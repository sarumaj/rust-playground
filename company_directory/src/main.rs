use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    println!("Enter department name and employee name separated by a space:");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            break;
        }

        let mut parts = input.split_whitespace();
        let department = parts.next().unwrap();
        let employee = parts.next().unwrap();
        add_employee(department, employee);
    }

    for department in list_departments() {
        println!("{}:", department);
        let employees = list_employees(&department).unwrap();
        for employee in employees {
            println!("  {}", employee);
        }
    }
}

static DEPARTMENTS: Lazy<Mutex<HashMap<String, Vec<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn add_employee(department: &str, employee: &str) {
    let mut departments = DEPARTMENTS.lock().unwrap();
    departments
        .entry(department.to_string())
        .or_insert(Vec::new())
        .push(employee.to_string());
}

fn list_departments() -> Vec<String> {
    let departments = DEPARTMENTS.lock().unwrap();
    departments.keys().cloned().collect()
}

fn list_employees(department: &str) -> Option<Vec<String>> {
    let departments = DEPARTMENTS.lock().unwrap();
    departments
        .get(department)
        .map(|employees| employees.clone())
}
