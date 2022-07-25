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

// 材質，Sync，Send継承
trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
}

// ランバート反射，わからなくなったら調べる．
struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let target = hit.p + hit.n + Vec3::random_in_unit_sphere();
        Some(ScatterInfo::new(Ray::new(hit.p, target - hit.p), self.albedo))
    }
}

// 鏡面反射する材質
struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let mut reflected = ray.direction.normalize().reflect(hit.n);
        reflected = reflected + self.fuzz * Vec3::random_in_unit_sphere();
        if reflected.dot(hit.n) > 0.0 {
            Some(ScatterInfo::new(Ray::new(hit.p, reflected), self.albedo))
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
}

impl HitInfo {
    fn new(t: f64, p: Point3, n:Vec3, m: Arc<dyn Material>) -> Self {
        Self { t, p, n, m }
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
                return Some(HitInfo::new(temp, p, (p - self.center) / self.radius, Arc::clone(&self.material)));
            }
            let temp = (-b + root) / (2.0 * a);
            if t0 < temp && temp < t1 {
                let p = ray.at(temp);
                return Some(HitInfo::new(temp, p, (p - self.center) / self.radius, Arc::clone(&self.material)));
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
    material: Option<Arc<dyn Material>>,
    shape: Option<Box<dyn Shape>>,
}

impl ShapeBuilder {
    fn new() -> Self {
        Self { material: None, shape: None }
    }

    fn lambertian(mut self, albedo: Color) -> Self {
        self.material = Some(Arc::new(Lambertian::new(albedo)));
        self
    }

    fn metal(mut self, albedo: Color, fuzz: f64) -> Self {
        self.material = Some(Arc::new(Metal::new(albedo, fuzz)));
        self
    }

    fn dielectric(mut self, ri:f64) -> Self {
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

struct RandomScene {
    world: ShapeList,
}

impl RandomScene {
    // 物体生成コンストラクタ
    fn new() -> Self {
        let mut world = ShapeList::new();

        world.push(ShapeBuilder::new()
            .lambertian(Color::new(0.5, 0.5, 0.5))
            .sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0)
            .build());
        
        // Small spheres
        for au in -11..11 {
            let a = au as f64;
            for bu in -11..11 {
                let b = bu as f64;
                let [rx, rz, material_choice] = Float3::random().to_array();
                let center = Point3::new(a + 0.9 * rx, 0.2, b + 0.9 * rz);
                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    world.push({
                        if material_choice < 0.8 {
                            let albedo = Color::random() * Color::random();
                            ShapeBuilder::new()
                                .lambertian(albedo)
                                .sphere(center, 0.2)
                                .build()
                        } else if material_choice < 0.95 {
                            let albedo = Color::random_limit(0.5, 1.0);
                            let fuzz = Float3::random_full().x();
                            ShapeBuilder::new()
                                .metal(albedo, fuzz)
                                .sphere(center, 0.2)
                                .build()
                        } else {
                            ShapeBuilder::new()
                                .dielectric(1.5)
                                .sphere(center, 0.2)
                                .build()
                        }
                    });
                }
            }
        }


        world.push(ShapeBuilder::new()
            .dielectric(1.5)
            .sphere(Point3::new(0.0, 1.0, 0.0), 1.0)
            .build());
        world.push(ShapeBuilder::new()
            .lambertian(Color::new(0.4, 0.2, 0.1))
            .sphere(Point3::new(-4.0, 1.0, 0.0), 1.0)
            .build());
        world.push(ShapeBuilder::new()
            .metal(Color::new(0.7, 0.6, 0.5), 0.0)
            .sphere(Point3::new(4.0, 1.0, 0.0), 1.0)
            .build());

        Self { world }
    }

        

    fn background(&self, d: Vec3) -> Color {
        let t = 0.5 * (d.normalize().y() + 1.0);
        Color::one().lerp(Color::new(0.5, 0.7, 1.0), t)
    }
}

impl SceneWithDepth for RandomScene {
    fn camera(&self) -> Camera {
        Camera::from_lookat(
            Point3::new(13.0, 2.0, 3.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::yaxis(),
            20.0,
            self.aspect(),
        )
    }

    // 反射率５０%
    fn trace(&self, ray: Ray, depth: usize) -> Color {
        let hit_info = self.world.hit(&ray, 0.001, f64::MAX);
        if let Some(hit) = hit_info {
            let scatter_info = if depth > 0 {hit.m.scatter(&ray, &hit) } else { None };
            if let Some(scatter) = scatter_info {
                scatter.albedo * self.trace(scatter.ray, depth - 1)
            } else {
                Color::zero()
            }
        } else {
            self.background(ray.direction)
        }
        // match hit_info {
        //     Some(hit) if depth > 0 => {
        //         if let Some(scatter) = hit.m.scatter(&ray, &hit) {
        //             scatter.albedo * self.trace(scatter.ray, depth-1)
        //         } else {
        //             Color::zero()
        //         }
        //     },
        //     Some(_) => { Color::zero() },
        //     None => { self.background_sky(ray.direction) },
        // }
    }
}

pub fn run() {
    render_aa_with_depth(RandomScene::new());
}