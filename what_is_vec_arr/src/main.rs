use std::fmt::Debug;

/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-03 13:28:13
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-04 23:23:08
 */
#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
struct Student {
    grade: f32,
    name: String,
    gender: Gender,
}
pub trait StudentTrait {
    fn new(name: String, grade: f32, gender: Gender) -> Self;
}

impl StudentTrait for Student {
    fn new(name: String, grade: f32, gender: Gender) -> Student {
        Student {
            name,
            grade,
            gender,
        }
    }
}

fn main() {
    let arr = [1, 2, 3];
    // 动态数组
    let mut vec: Vec<i32> = Vec::new();

    vec.push(1);

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        if (*i == 1) {
            *i += 10
        }
    }

    println!("{:?}", v);

    let v1 = &vec![1, 2, 3];
    let v3 = v1.into_iter().map(|item| item * item).collect::<Vec<i32>>();
    let v4 = v1
        .into_iter()
        .map(|item| item * item * item)
        .collect::<Vec<i32>>();
    // let v2 = v1
    //     .into_iter()
    //     .filter_map(|i| {
    //         if (i < 3) {
    //             return Some(2);
    //         }
    //         Some(i)
    //     })
    //     .collect::<Vec<i32>>();

    let v2: Vec<_> = v1
        .into_iter()
        .filter(|&x| x - 2 > 0)
        .map(|x| x * x)
        .collect();

    println!("v3 -{:?}, v4- {:?},  v2{:?}", v3, v4, v2);

    let xiaoming = Student::new(String::from("xiaoming"), 85.0, Gender::Male);

    let xiaohong = Student::new(String::from("xiaohong"), 75.2, Gender::Female);
    let xiaokang = Student::new(String::from("xiaokang"), 60.0, Gender::Male);

    let mut student_vec: Vec<Student> = vec![];
    student_vec.push(xiaoming);
    student_vec.push(xiaohong);
    student_vec.push(xiaokang);

    let good_student: Vec<_> = get_good_student(&student_vec);

    println!("Good Student is {:?}", good_student);
}

fn get_good_student(good_student: &Vec<Student>) -> Vec<&(impl StudentTrait + Debug)> {
    good_student
        .into_iter()
        .filter(|x| x.grade as i32 - 80 > 0)
        .collect()
}
