use std::io::{stdin, Write};

fn main() {

    let mut file_name = String::new();
    println!("Enter the name of the result file, e.g. my_results.txt");
    stdin().read_line(&mut file_name).expect("Not a correct line");
    let trim_name = file_name.trim();
    let file_creation = format!("{}{}", "./", trim_name);
    let mut results = std::fs::File::create(file_creation).expect("Create failed");

    let mut body1_mass = String::new();
    let mut body1_radius = String::new();
    let mut body2_mass = String::new();
    let mut body2_radius = String::new();
    let mut dist_between = String::new();
    let g_const: f64 = 6.67430e-11;
    let mut body_speed1: f64 = 0.0;
    let mut body_speed2: f64 = 0.0;

    println!("Enter the mass of body1");
    stdin().read_line(&mut body1_mass).expect("not a correct line");
    let sm1 = body1_mass.trim();
    let float_mass1 = sm1.parse::<f64>().expect("Not a correct number");

    println!("Enter radius of body1");
    stdin().read_line(&mut body1_radius).expect("not a correct line");
    let sr1 = body1_radius.trim();
    let float_radius1 = sr1.parse::<f64>().expect("Not a correct number");

    println!("Enter the mass of body2");
    stdin().read_line(&mut body2_mass).expect("not a correct line");
    let sm2 = body2_mass.trim();
    let float_mass2 = sm2.parse::<f64>().expect("Not a correct number");

    println!("Enter radius of body2");
    stdin().read_line(&mut body2_radius).expect("not a correct line");
    let sr2 = body2_radius.trim();
    let float_radius2 = sr2.parse::<f64>().expect("Not a correct number");

    println!("Enter the distance between the two");
    stdin().read_line(&mut dist_between).expect("not a correct line");
    let db = dist_between.trim();
    let mut float_distance = db.parse::<f64>().expect("not a correct number");


    let mut dist_minus_radius = float_distance - float_radius1 - float_radius2;
    let mut a_count: u32 = 0;
    while dist_minus_radius > 0.0 {
        let the_force = g_const * (float_mass1 * float_mass2) / (float_distance * float_distance);
        let accel1 = the_force / float_mass1;
        body_speed1 += accel1;
        let accel2 = the_force / float_mass2;
        body_speed2 += accel2;
        float_distance = float_distance - body_speed1 - body_speed2;
        dist_minus_radius = dist_minus_radius - body_speed1 - body_speed2;
        a_count += 1;

        let write_string = format!("Speed1 {} Accel1 {} Speed2 {} Accel2 {}  Distance {} Seconds {}\n",
             body_speed1, accel1, body_speed2, accel2, dist_minus_radius, a_count);
        results.write_all(write_string.as_bytes()).expect("Write failed");
    
    }
    let the_seconds = a_count as f64;
    let the_hours = the_seconds / 3600.0;
    let the_days = the_hours / 24.0;
    let year_365 = the_days / 365.0;
    println!("It takes {} seconds or {} hours or {} days or {} years (365 days)", the_seconds, the_hours, the_days, year_365);

}
