extern crate nalgebra as na;
extern crate ncollide2d;
extern crate nphysics2d;
extern crate nphysics_testbed2d;

use na::{Isometry2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::{
    object::{BodyHandle, Material},
    volumetric::Volumetric,
    world::World,
};
use nphysics_testbed2d::Testbed;

const COLLIDER_MARGIN: f32 = 0.01;

fn main() {
    /*
     * World
     */
    #[cfg(not(target_arch = "wasm32"))]
    let mut world: World<f32> = World::new();
    #[cfg(target_arch = "wasm32")]
    let mut world: World<f32> = World::new(|| 0.0);

    world.set_gravity(Vector2::new(0.0, -9.81));

    /*
     * Ground
     */
    let ground_radx = 25.0;
    let ground_rady = 1.0;
    let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
        ground_radx - COLLIDER_MARGIN,
        ground_rady - COLLIDER_MARGIN,
    )));

    let ground_pos = Isometry2::new(Vector2::y() * -ground_rady, na::zero());
    world.add_collider(
        COLLIDER_MARGIN,
        ground_shape,
        BodyHandle::ground(),
        ground_pos,
        Material::default(),
    );

    /*
     * Create the boxes
     */
    let num = 25;
    let rad = 0.1;
    let shift = 2.0 * rad;
    let centerx = shift * (num as f32) / 2.0;

    let geom =
        ShapeHandle::new(Cuboid::new(Vector2::repeat(rad - COLLIDER_MARGIN)));
    let inertia = geom.inertia(1.0);
    let center_of_mass = geom.center_of_mass();

    for i in 0usize..num {
        for j in i..num {
            let fj = j as f32;
            let fi = i as f32;
            let x = (fi * shift / 2.0) + (fj - fi) * 2.0 * rad - centerx;
            let y = fi * 2.0 * rad + rad;

            /*
             * Create the rigid body.
             */
            let pos = Isometry2::new(Vector2::new(x, y), 0.0);
            let handle = world.add_rigid_body(pos, inertia, center_of_mass);

            /*
             * Create the collider.
             */
            world.add_collider(
                COLLIDER_MARGIN,
                geom.clone(),
                handle,
                Isometry2::identity(),
                Material::default(),
            );
        }
    }

    /*
     * Run the simulation.
     */
    let testbed = Testbed::new(world);
    testbed.run();
}
