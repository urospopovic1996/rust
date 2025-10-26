fn main() {
    // 1. Array to store three tuples, each containing name, age and average grade
    let student1 = ("Marko", 23, 10.0);
    let student2 = ("Marija", 22, 8.0);
    let student3 = ("Milan", 21, 8.5);

    let students = [student1, student2, student3];

    // 2. Print a formatted table of all students
    let mut total_grade_average = 0.0;
    
    println!("Name\tAge\tAverage Grade\n");
    for student in students {
        let (name, age, grade_average) = student;
        println!("{}\t{}\t{}\n", name, age, grade_average);
        total_grade_average += grade_average;
    }

    // 3. Calculate the average grade of all students
    let average_grade = total_grade_average / students.len() as f32;
    println!("Average grade of all students: {}", average_grade);
}
