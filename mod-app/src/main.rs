mod foo;
mod fooz;

fn main() {
    foo::function();
    foo::bar::function();
    fooz::function();
}
