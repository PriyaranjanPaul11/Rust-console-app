use std::io;
use genpdf::{elements, style};
use genpdf::Element;

struct Student {
    name: String,
    total_marks: f32,
    num_subjects: u32,
}

impl Student {
    fn average(&self) -> f32 {
        self.total_marks / self.num_subjects as f32
    }

    fn grade(&self) -> &'static str {
        let avg = self.average();
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
}

fn main() {
    let mut name = String::new();
    let mut total_marks = String::new();
    let mut num_subjects = String::new();

    println!("Enter Student Name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter Total Marks:");
    io::stdin().read_line(&mut total_marks).unwrap();

    println!("Enter Number of Subjects:");
    io::stdin().read_line(&mut num_subjects).unwrap();

    let student = Student {
        name: name.trim().to_string(),
        total_marks: total_marks.trim().parse().unwrap_or(0.0),
        num_subjects: num_subjects.trim().parse().unwrap_or(1),
    };

    let average = student.average();
    let grade = student.grade();

    println!("\n--- Report Card ---");
    println!("Name       : {}", student.name);
    println!("Average    : {:.2}", average);
    println!("Grade      : {}", grade);

    generate_pdf_report(&student, average, grade);
    println!("\nPDF generated: report_card.pdf");
}

fn generate_pdf_report(student: &Student, average: f32, grade: &str) {
    let font_family = genpdf::fonts::from_files(
        "./fonts",              
        "LiberationSans",       
        None,                   
    ).expect("Failed to load font");

    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Report Card");

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    let heading = elements::Paragraph::new("Student Report Card")
        .styled(style::Style::new().bold().with_font_size(20));

    let mut table = elements::TableLayout::new(vec![1, 2]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));

    table.row()
        .element(elements::Paragraph::new("Name"))
        .element(elements::Paragraph::new(&student.name))
        .push()
        .unwrap();

    table.row()
        .element(elements::Paragraph::new("Total Marks"))
        .element(elements::Paragraph::new(format!("{:.2}", student.total_marks)))
        .push()
        .unwrap();

    table.row()
        .element(elements::Paragraph::new("Subjects"))
        .element(elements::Paragraph::new(student.num_subjects.to_string()))
        .push()
        .unwrap();

    table.row()
        .element(elements::Paragraph::new("Average"))
        .element(elements::Paragraph::new(format!("{:.2}", average)))
        .push()
        .unwrap();

    table.row()
        .element(elements::Paragraph::new("Grade"))
        .element(elements::Paragraph::new(grade))
        .push()
        .unwrap();

    doc.push(heading);
    doc.push(elements::Break::new(1));
    doc.push(table);

    doc.render_to_file("report_card.pdf").expect("Failed to write PDF");
}
