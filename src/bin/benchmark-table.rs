use datakit::table::*;
use datakit::value::*;
use rand::prelude::*;
use serde::Serialize;
use std::time::Instant;

#[derive(Serialize, Debug)]
struct BenchmarkResult {
    pub case: String,
    pub size: usize,
    pub expression: String,
    pub duration_micros: u128,
    pub success: bool,
}

macro_rules! bench {
    ($($case:expr, $size:expr => $fn:expr)?) => {
        $(
            {
                let start_time = Instant::now();
                let success = match $fn {
                    Ok(_) => true,
                    Err(_) => false,
                };
                let duration_micros = start_time.elapsed().as_micros();
                let expression = String::from(stringify!($fn));
                let case = String::from($case);
                let size = $size;

                BenchmarkResult {
                    case,
                    size,
                    expression,
                    duration_micros,
                    success
                }
            }
        )?
    };
}

fn generate_random_person_name<R>(rng: &mut R) -> Value
where
    R: Rng,
{
    let boy_names = vec![
        "Liam",
        "Noah",
        "William",
        "James",
        "Oliver",
        "Benjamin",
        "Elijah",
        "Lucas",
        "Mason",
        "Logan",
        "Alexander",
        "Ethan",
        "Jacob",
        "Michael",
        "Daniel",
        "Henry",
        "Jackson",
        "Sebastian",
        "Aiden",
        "Matthew",
        "Samuel",
        "David",
        "Joseph",
        "Carter",
        "Owen",
        "Wyatt",
        "John",
        "Jack",
        "Luke",
        "Jayden",
        "Dylan",
        "Grayson",
        "Levi",
        "Isaac",
        "Gabriel",
        "Julian",
        "Mateo",
        "Anthony",
        "Jaxon",
        "Lincoln",
        "Joshua",
        "Christopher",
        "Andrew",
        "Theodore",
        "Caleb",
        "Ryan",
        "Asher",
        "Nathan",
        "Thomas",
        "Leo",
        "Isaiah",
        "Charles",
        "Josiah",
        "Hudson",
        "Christian",
        "Hunter",
        "Connor",
        "Eli",
        "Ezra",
        "Aaron",
        "Landon",
        "Adrian",
        "Jonathan",
        "Nolan",
        "Jeremiah",
        "Easton",
        "Elias",
        "Colton",
        "Cameron",
        "Carson",
        "Robert",
        "Angel",
        "Maverick",
        "Nicholas",
        "Dominic",
        "Jaxson",
        "Greyson",
        "Adam",
        "Ian",
        "Austin",
        "Santiago",
        "Jordan",
        "Cooper",
        "Brayden",
        "Roman",
        "Evan",
        "Ezekiel",
        "Xavier",
        "Jose",
        "Jace",
        "Jameson",
        "Leonardo",
        "Bryson",
        "Axel",
        "Everett",
        "Parker",
        "Kayden",
        "Miles",
        "Sawyer",
        "Jason",
    ];

    let girl_names = vec![
        "Emma",
        "Olivia",
        "Ava",
        "Isabella",
        "Sophia",
        "Charlotte",
        "Mia",
        "Amelia",
        "Harper",
        "Evelyn",
        "Abigail",
        "Emily",
        "Elizabeth",
        "Mila",
        "Ella",
        "Avery",
        "Sofia",
        "Camila",
        "Aria",
        "Scarlett",
        "Victoria",
        "Madison",
        "Luna",
        "Grace",
        "Chloe",
        "Penelope",
        "Layla",
        "Riley",
        "Zoey",
        "Nora",
        "Lily",
        "Eleanor",
        "Hannah",
        "Lillian",
        "Addison",
        "Aubrey",
        "Ellie",
        "Stella",
        "Natalie",
        "Zoe",
        "Leah",
        "Hazel",
        "Violet",
        "Aurora",
        "Savannah",
        "Audrey",
        "Brooklyn",
        "Bella",
        "Claire",
        "Skylar",
        "Lucy",
        "Paisley",
        "Everly",
        "Anna",
        "Caroline",
        "Nova",
        "Genesis",
        "Emilia",
        "Kennedy",
        "Samantha",
        "Maya",
        "Willow",
        "Kinsley",
        "Naomi",
        "Aaliyah",
        "Elena",
        "Sarah",
        "Ariana",
        "Allison",
        "Gabriella",
        "Alice",
        "Madelyn",
        "Cora",
        "Ruby",
        "Eva",
        "Serenity",
        "Autumn",
        "Adeline",
        "Hailey",
        "Gianna",
        "Valentina",
        "Isla",
        "Eliana",
        "Quinn",
        "Nevaeh",
        "Ivy",
        "Sadie",
        "Piper",
        "Lydia",
        "Alexa",
        "Josephine",
        "Emery",
        "Julia",
        "Delilah",
        "Arianna",
        "Vivian",
        "Kaylee",
        "Sophie",
        "Brielle",
        "Madeline",
    ];

    let possible_names = if rand::random() {
        &girl_names
    } else {
        &boy_names
    };

    let mut names: Vec<String> = Vec::new();
    for _ in 0..rng.gen_range(1, 4) {
        if let Some(name) = possible_names.choose(rng) {
            names.push(String::from(*name));
        }
    }
    Value::Text(names.join(" "))
}

fn generate_random_flavor<R>(rng: &mut R) -> Value
where
    R: Rng,
{
    let possible_flavours = vec![
        ("Apple", 10),
        ("Cherry", 9),
        ("Blueberry", 6),
        ("Bannana", 4),
        ("Beef", 1),
    ];

    let flavor = possible_flavours
        .choose_weighted(rng, |item| item.1)
        .unwrap()
        .0;

    Value::Text(String::from(flavor))
}

fn generate_random_number_of_pies_eaten<R>(rng: &mut R) -> Value
where
    R: Rng,
{
    let n: i64 = rng.gen_range(0, 10);
    Value::Number(Numeric::Integer(n))
}

fn generate_random_table<R>(rng: &mut R, number_of_rows: usize) -> Table
where
    R: Rng,
{
    let schema = Schema::from_tuples(vec![
        (
            "Name",
            ValueContract {
                expected_type: TypeConstraint::IsType(ValueType::Text),
                value_constraints: vec![ValueConstraint::MaximumLength(25)],
            },
        ),
        (
            "FavoritePie",
            ValueContract {
                expected_type: TypeConstraint::IsType(ValueType::Text),
                value_constraints: vec![ValueConstraint::OneOf(vec![
                    Value::Text(String::from("Apple")),
                    Value::Text(String::from("Cherry")),
                    Value::Text(String::from("Blueberry")),
                ])],
            },
        ),
        (
            "PiesEaten",
            ValueContract {
                expected_type: TypeConstraint::IsType(ValueType::Number),
                value_constraints: vec![ValueConstraint::Maximum(9.into())],
            },
        ),
    ]);

    let mut table = Table::from_schema(&schema);

    for _ in 0..number_of_rows {
        table
            .add_row(&vec![
                generate_random_person_name(rng),
                generate_random_flavor(rng),
                generate_random_number_of_pies_eaten(rng),
            ])
            .unwrap();
    }

    table
}

fn main() {
    let mut writer = csv::Writer::from_path("benchmark-table.csv").unwrap();

    let mut rng = rand::thread_rng();

    for n in 0..20000 {
        console::Term::stdout().clear_screen().unwrap();
        print!("n = ");
        println!("{}", console::style(n).red());
        let table = generate_random_table(&mut rng, n);
        let seq_bench = bench!("Sequential validation", n => table.validate_table());
        let par_bench = bench!("Parallel validation", n => table.validate_table_par());
        writer.serialize(seq_bench).unwrap();
        writer.serialize(par_bench).unwrap();
    }
}
