fn main() {
    println!("Hello, world!");
    greet_world();
    scary_function();
}

fn greet_world() {
    println!("Multi Language Hello World \n");
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions {
        println!("{}", &region);
    }
    println!("\n")
}

fn scary_function() {
    let penguin_data = "\
   common name,20
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,11
   ";

    println!("Scary Function \n");
    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if (i == 0 && record.trim().len() == 0) || (i > 0 && record.trim().len() == 0) {
            continue;
        }

        let fields: Vec<_> = record.split(",").map(|x| x.trim()).collect();
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
    println!("\n");
}
