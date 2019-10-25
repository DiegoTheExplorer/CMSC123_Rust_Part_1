use std::io; //lib for std I/O


struct Subject{
    class_section: String,
    course_code: String
}

struct Student{
    name: String,
    s_num: String,
    subjects: Vec<Subject>
}

fn printMainMenu(){
    println!("[Main Menu]");
    println!("[1] Add student");
    println!("[2] View ALL Students");
    println!("[3] Add subject to student");
    println!("[4] Delete student");
    println!("[5] Exit");
}

fn addStudent(mut stdList: &mut Vec<Student>){
    println!("[Add Student]");
    println!();
    println!("Enter student information");

    println!("Enter student's name: ");//Input for student's name
    let mut stdName = String::new();
    io::stdin()
        .read_line(&mut stdName)
        .expect("failed to read from stdin");

    println!("Enter student's student number: ");//Input for student's std-num
    let mut stdNum = String::new();
    io::stdin()
        .read_line(&mut stdNum)
        .expect("failed to read from stdin");

    let name = &stdName;
    let num = &stdNum;
    let listStd = stdList.iter();
    let mut lstStdnt:Vec<Student> = Vec::new();
    let mut exists = false;

    let mut stdHldr:Student = Student{
        name: "0".to_string(),
        s_num: "0".to_string(),
        subjects: Vec::new()
    };

    if stdList.is_empty(){//Checking whether the Student Vector is empty
        println!("Saving student....");
        let newStd:Student = Student{//Student initialization
            name: name.to_string(),
            s_num: num.to_string(),
            subjects: Vec::new()
        };
        stdList.push(newStd);
        println!("Student saved successfully");
    }else {
        for std in listStd {//Checking for duplicates
            if std.s_num == num.to_string(){
                println!("Student with student number {} already exists", stdNum);
                exists = true;
                break;
            }
        }
        if !exists{
            println!("Saving student....");
            let newStd:Student = Student{//Student initialization
                name: name.to_string(),
                s_num: num.to_string(),
                subjects: Vec::new()
            };
            stdHldr = newStd;
            println!("Student saved successfully");
            stdList.push(stdHldr);
            return;
        }
    }
}

fn viewAllStds(mut stdList: &mut Vec<Student>){
    let listStd = stdList.iter();

    if stdList.is_empty(){
        println!("There are no students in the directory");
    }else{
        for std in listStd {//iterate through students
            println!("--------------------------------------------------");
            println!("Student Name: {}", std.name);
            println!("Student Number: {}", std.s_num);
            println!("[Subjects]:");
            if !std.subjects.is_empty(){
                let listSubj = std.subjects.iter();
                for subj in listSubj{//iterate through a student's subjects
                    println!("  ****************************************");
                    println!("  Class Section: {}", subj.class_section);
                    println!("  Course Code: {}", subj.course_code);
                    println!("  ****************************************");
                }
            }else{
                println!("  No Subjects Enlisted");
            }
            println!("--------------------------------------------------");
        }
    }
}

fn addSubjToStd(mut stdList: &mut Vec<Student>){
    let mut exists = false;

    println!("[Add Subject to Student]");
    println!();
    println!("Enter student information");

    println!("Enter student's student number: ");//Input for student's std-num
    let mut stdNum = String::new();
    io::stdin()
        .read_line(&mut stdNum)
        .expect("failed to read from stdin");
    let num = &stdNum;

    if stdList.is_empty(){//Checking whether the Student Vector is empty
        println!("There are no students in the directory");
    }else {
        let listStd = stdList.iter_mut();
        for std in listStd {//Checking for duplicates
            if std.s_num == num.to_string(){
                exists = true;
                println!("Student Found!");
                println!("Enter Subject Information");

                println!("Enter Class Section:");//Input for subject's class_section
                let mut classSection = String::new();
                io::stdin()
                    .read_line(&mut classSection)
                    .expect("failed to read from stdin");
                let section = &classSection;

                println!("Enter Course Code:");//Input for subject's course_code
                let mut courseCode = String::new();
                io::stdin()
                    .read_line(&mut courseCode)
                    .expect("failed to read from stdin");
                let code = &courseCode;

                let mut newSubj:Subject = Subject{
                    class_section: section.to_string(),
                    course_code: code.to_string()
                };

                let student = std;
                student.subjects.push(newSubj);
                break;
            }
        }
        if !exists{
            println!("Student with entered student number does not exist");
        }
    }
}

fn deleteStudent(mut stdList: &mut Vec<Student>){
    println!("[Delete Student]");
    println!();

    if stdList.is_empty(){//Checking whether the Student Vector is empty
        println!("Threre are no students in the directory");
    }else {

        let mut exists = false;
        println!("Enter student the student-to-be-deleted's information");

        println!("Enter student's student number: ");//Input for student's std-num
        let mut stdNum = String::new();
        io::stdin()
            .read_line(&mut stdNum)
            .expect("failed to read from stdin");

        let num = &stdNum;
        let listStd = stdList.iter();
        let mut lstStdnt:Vec<Student> = Vec::new();
        let mut exists = false;
        let mut index = 0;

        for std in listStd {//Checking for duplicates
            if std.s_num == num.to_string(){
                println!("Student Found!");
                println!("Deleting Student");
                exists = true;
                break;
            }
            index = index + 1;
        }

        if !exists{
            println!("Student with entered student number does not exist");
        }else{
            stdList.remove(index);
        }
    }
}

fn main() {
    let mut stdList:Vec<Student> = Vec::new();
    let mut strChoice:String = String::new();
    let mut choice:i32 = 0;

    println!("Welcome to the Rust Port of Siyete!");
    loop{

        // choice = 0;
        // printMainMenu();
        // choice =  input("Enter your choice: ");
        // println!("You chose: {}" , choice);

        printMainMenu();
        println!("Enter your choice: ");
        let mut choiceStr = String::new();
        io::stdin()
            .read_line(&mut choiceStr)
            .expect("failed to read from stdin");

        let choice = choiceStr.trim();
        match choice.parse::<u32>() {
            Ok(i) => println!("Your choice: {}", i),
            Err(..) => println!("this was not an integer: {}", choice),
        };

        match choice.parse::<u32>() {
            Ok(1) => addStudent(&mut stdList),
            Ok(2) => viewAllStds(&mut stdList),
            Ok(3) => addSubjToStd(&mut stdList),
            Ok(4) => deleteStudent(&mut stdList),
            Ok(5) => break,
            Ok(_) => println!("Invalid Input"),
            Err(..) => println!("Invalid Input"),
        }

    }

}
