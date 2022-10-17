mod project_mgmt;

use project_mgmt as pm;
use project_mgmt::project;

fn main() {
    println!("Hello, world!");
    pm::init();
    pm::new_project();
    project::Project {};
}
