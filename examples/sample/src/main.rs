use plpgsql_maker::hi;

struct InParams {
    name: String,
    id: i64,
}

struct OutParams {
    name: String,
    id: i64,
}

struct Body {
}

fn main() {
    println!("{}", hi());
}
