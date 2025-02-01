use std::fs;

fn main() {

 let content  = fs::read_to_string("src/input.txt").unwrap();
 let data: Vec<_> = content.lines().collect();
 println!("data: {:?}", data);
 
 let mut first_numbers: Vec<i32> = Vec::new();
 let mut second_numbers: Vec<i32> = Vec::new();

 for line in data {
     let numbers: Vec<&str> = line.split_whitespace().collect();

     if numbers.len() == 2 {
         if let Ok(first) = numbers[0].parse::<i32>() {
             first_numbers.push(first);
         }
         if let Ok(second) = numbers[1].parse::<i32>() {
             second_numbers.push(second);
         }
     }
 }

 first_numbers.sort();
 second_numbers.sort();

 let mut sum: i32 = 0;

 for (a,b) in first_numbers.iter().zip(second_numbers.iter()) {
     println!("a: {}, b: {}", a, b);
     let result = (a - b).abs();
     sum += result;
 }

 println!("first_numbers: {:?}, second_numbers: {:?}", first_numbers, second_numbers);

 println!("sum: {}", sum);

}

