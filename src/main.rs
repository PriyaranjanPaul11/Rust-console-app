use printpdf::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use text_io::read;

fn calculate_average(total_marks: f64, subjects: u32) -> f64 {
    total_marks / subjects as f64
}

fn assign_grade(avg: f64) -> &'static str {
    if avg >= 90.0 {
        "A"
    } else if avg >= 75.0 {
        "B"
    } else if avg >= 60.0 {
        "C"
    } else {
        "D"
    }
}

fn main() {
    println!("Enter student name:");
    let name: String = read!("{}\n");

    println!("Enter total marks:");
    let total_marks: f64 = read!();

    println!("Enter number of subjects:");
    let subjects: u32 = read!();

    let average = calculate_average(total_marks, subjects);
    let grade = assign_grade(average);

    println!("\n--- Student Report ---");
    println!("Name: {}", name);
    println!("Total Marks: {}", total_marks);
    println!("Subjects: {}", subjects);
    println!("Average: {:.2}", average);
    println!("Grade: {}", grade);

    generate_pdf(&name, total_marks, subjects, average, grade);
    println!("PDF Report card generated as report_card.pdf");
}

fn generate_pdf(name: &str, marks: f64, subjects: u32, avg: f64, grade: &str) {
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");

    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    let lines = vec![
        format!("--- Student Report Card ---"),
        format!("Name: {}", name),
        format!("Total Marks: {}", marks),
        format!("Subjects: {}", subjects),
        format!("Average: {:.2}", avg),
        format!("Grade: {}", grade),
    ];

    let mut y = 270.0;
    for line in lines {
        current_layer.use_text(line, 14.0, Mm(20.0), Mm(y), &font);
        y -= 10.0;
    }

    doc.save(&mut BufWriter::new(File::create("report_card.pdf").unwrap()))
        .unwrap();
}