pub mod components {
    automod::dir!(pub "src/components");
}


fn main() {
    use components::Header;
    
    println!("{:?}", Header::default());
}
