use actix_web_static_files::resource_dir;

fn main() {
    resource_dir("./../../ui/build").build().unwrap();
}