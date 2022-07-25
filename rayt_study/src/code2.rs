use crate::rayt_mod::*;

// 光の散乱
struct ScatterInfo {
    ray: Ray,
    albedo: Color,
}

impl ScatterInfo {
    fn new(ray: Ray, albedo: Color) -> Self {
        Self { ray, albedo }
    }
}

// Texture
// u,vはテクスチャ座標，pはピクセルの位置情報
trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

// 手続き型テクスチャ，カラー（反射率)を持つ．
struct ColorTexture {
    color: Color,
}

impl ColorTexture {
    const fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for ColorTexture {
    fn value(&self, _u:f64, _v: f64, _p: Point3) -> Color {
        self.color
    }
}

// 縞模様テクスチャ,freqは縞のfreq
struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
    freq: f64,
}

impl CheckerTexture {
    fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>, freq: f64) -> Self {
        Self { odd, even, freq }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = p.iter().fold(1.0, |acc, x| acc * (x * self.freq).sin());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

// 材質，Sync，Send継承
trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
}

// ランバート反射，わからなくなったら調べる．
struct Lambertian {
    albedo: Box<dyn Texture>,
}

impl Lambertian {
    fn new(albedo: Box<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let target = hit.p + hit.n + Vec3::random_in_unit_sphere();
        let albedo = self.albedo.value(hit.u, hit.v, hit.p);
        Some(ScatterInfo::new(Ray::new(hit.p, target - hit.p), albedo))
    }
}

// 鏡面反射する材質
struct Metal {
    albedo: Box<dyn Texture>,
    fuzz: f64,
}

impl Metal {
    fn new(albedo: Box<dyn Texture>, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let mut reflected = ray.direction.normalize().reflect(hit.n);
        reflected = reflected + self.fuzz * Vec3::random_in_unit_sphere();
        if reflected.dot(hit.n) > 0.0 {
            let albedo = self.albedo.value(hit.u, hit.v, hit.p);
            Some(ScatterInfo::new(Ray::new(hit.p, reflected), albedo))
        } else {
            None
        }
    }
}

// 誘導体媒質
struct Dielectric {
    ri: f64,
}

impl Dielectric {
    const fn new(ri: f64) -> Self {
        Self { ri }
    }
    
    fn schlick(cosine: f64, ri: f64) -> f64 {
        let r0 = ((1.0 - ri) / ( 1.0 + ri )).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let reflected = ray.direction.reflect(hit.n);
        let (outward_normal, ni_over_nt, cosine) = {
            let dot = ray.direction.dot(hit.n);
            if dot > 0.0 {
                (-hit.n, self.ri, self.ri * dot / ray.direction.length())
            } else {
                (hit.n, self.ri.recip(), -dot / ray.direction.length())
            }
        };

        if let Some(refracted) = (-ray.direction).refract(outward_normal, ni_over_nt) {
            if Vec3::random_full().x() > Self::schlick(cosine, self.ri) {
                return Some(ScatterInfo::new(Ray::new(hit.p, refracted), Color::one()));
            }
        }

        Some(ScatterInfo::new(Ray::new(hit.p, reflected), Color::one()))
    }
}

// 当たり判定

struct HitInfo {
    t: f64,
    p: Point3,
    n: Vec3,
    m: Arc<dyn Material>,
    u: f64,
    v: f64,
}

impl HitInfo {
    fn new(t: f64, p: Point3, n:Vec3, m: Arc<dyn Material>, u: f64, v: f64) -> Self {
        Self { t, p, n, m, u, v }
    }
}

// 物体トレイト．Syncトレイト継承．
trait Shape: Sync {
    // 衝突関数．t0とt1は光線の衝突範囲．
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo>;
}

struct Sphere{
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius.powi(2);
        let d = b * b - 4.0 * a * c;
        if d > 0.0 {
            let root = d.sqrt();
            let temp = (-b - root) / (2.0 * a);
            if t0 < temp && temp < t1 {
                let p = ray.at(temp);
                return Some(HitInfo::new(temp, p, (p - self.center) / self.radius, Arc::clone(&self.material), 0.0, 0.0));
            }
            let temp = (-b + root) / (2.0 * a);
            if t0 < temp && temp < t1 {
                let p = ray.at(temp);
                return Some(HitInfo::new(temp, p, (p - self.center) / self.radius, Arc::clone(&self.material), 0.0, 0.0));
            }
        }

        None
    }
}

// 物体リスト．複数物体の管理．
struct ShapeList {
    pub objects: Vec<Box<dyn Shape>>,
}

impl ShapeList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn push(&mut self, object: Box<dyn Shape>){
        self.objects.push(object);
    }
}

impl Shape for ShapeList {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let mut hit_info: Option<HitInfo> = None;
        let mut closest_so_far = t1;
        for object in &self.objects {
            if let Some(info) = object.hit(ray, t0, closest_so_far) {
                closest_so_far = info.t;
                hit_info = Some(info);
            }
        }

        hit_info
    }
}

struct ShapeBuilder {
    texture: Option<Box<dyn Texture>>,
    material: Option<Arc<dyn Material>>,
    shape: Option<Box<dyn Shape>>,
}

impl ShapeBuilder {
    fn new() -> Self {
        Self { texture: None, material: None, shape: None }
    }

    fn color_texture(mut self, color: Color) -> Self {
        self.texture = Some(Box::new(ColorTexture::new(color)));
        self
    }

    fn checker_texture(mut self, odd_color: Color, even_color: Color, freq: f64) -> Self {
        self.texture = Some(Box::new(CheckerTexture::new(
            Box::new(ColorTexture::new(odd_color)),
            Box::new(ColorTexture::new(even_color)),
            freq,
        )));
        self
    }

    fn lambertian(mut self) -> Self {
        self.material = Some(Arc::new(Lambertian::new(self.texture.unwrap())));
        self.texture = None;
        self
    }

    fn metal(mut self, fuzz: f64) -> Self {
        self.material = Some(Arc::new(Metal::new(self.texture.unwrap(), fuzz)));
        self.texture = None;
        self
    }

    fn dielectric(mut self, ri: f64) -> Self {
        self.material = Some(Arc::new(Dielectric::new(ri)));
        self
    }

    fn sphere(mut self, center: Point3, radius: f64) -> Self {
        self.shape = Some(Box::new(Sphere::new(center, radius, self.material.unwrap())));
        self.material = None;
        self
    }

    fn build(self) -> Box<dyn Shape> {
        self.shape.unwrap()
    }
}

struct SimpleScene {
    world: ShapeList,
}

impl SimpleScene {
    fn new() -> Self {
        let mut world = ShapeList::new();
        world.push(ShapeBuilder::new()
            .color_texture(Color::new(0.1, 0.2, 0.5))
            .lambertian()
            .sphere(Point3::new(0.6, 0.0, -1.0), 0.5)
            .build());
        world.push(ShapeBuilder::new()
            .color_texture(Color::new(0.8, 0.8, 0.8))
            .metal(0.4)
            .sphere(Point3::new(-0.6, 0.0, -1.0), 0.5)
            .build());
        world.push(ShapeBuilder::new()
            .checker_texture(
                Color::new(0.8, 0.8, 0.0),
                Color::new(0.8, 0.2, 0.0),
                10.0
            )
            .lambertian()
            .sphere(Point3::new(0.0, -100.5, -1.0), 100.0)
            .build());
        
        Self { world }
    }

    fn background(&self, d: Vec3) -> Color {
        let t = 0.5 * (d.normalize().y() + 1.0);
        Color::one().lerp(Color::new(0.5, 0.7, 1.0), t)
    }
}

impl SceneWithDepth for SimpleScene {
    fn camera(&self) -> Camera {
        Camera::new(
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            Vec3::new(-2.0, -1.0, -1.0),
        )
    }

    fn trace(&self, ray: Ray, depth: usize) -> Color {
        let hit_info = self.world.hit(&ray, 0.001, f64::MAX);
        if let Some(hit) = hit_info {
            let scatter_info = if depth > 0 { hit.m.scatter(&ray, &hit) } else { None };
            if let Some(scatter) = scatter_info {
                scatter.albedo * self.trace(scatter.ray, depth - 1)
            } else {
                Color::zero()
            }
        } else {
            self.background(ray.direction)
        }
    }
}

pub fn run() {
    render_aa_with_depth(SimpleScene::new());
}