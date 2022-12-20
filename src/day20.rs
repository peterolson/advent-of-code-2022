use crate::input::read_input;

pub fn part1() {
    let lines = read_input("day20");

    let mut numbers : Vec<isize> = Vec::new();
    for line in lines {
        let number = line.parse::<isize>().unwrap();
        numbers.push(number);
    }

    let mixed = mix(&numbers, 1);

    let index_of_0 = mixed.iter().position(|x| *x == 0).unwrap();

    let n_1000 = get_at(&mixed, index_of_0 as isize + 1000);
    let n_2000 = get_at(&mixed, index_of_0 as isize + 2000);
    let n_3000 = get_at(&mixed, index_of_0 as isize + 3000);



    let sum = n_1000 + n_2000 + n_3000;

    println!("Day 20 Part 1: {}", sum);

    let decrypted_n : Vec<isize> = numbers.clone().iter().map(|x| x * 811589153).collect();

    let mixed = mix(&decrypted_n, 10);

    let index_of_0 = mixed.iter().position(|x| *x == 0).unwrap();

    let n_1000 = get_at(&mixed, index_of_0 as isize + 1000);
    let n_2000 = get_at(&mixed, index_of_0 as isize + 2000);
    let n_3000 = get_at(&mixed, index_of_0 as isize + 3000);



    let sum = n_1000 + n_2000 + n_3000;

    println!("Day 20 Part 2: {}", sum);

}

fn get_at(numbers: &Vec<isize>, index: isize) -> isize {
    return numbers[get_index(numbers.len(), index)];
}

fn get_index(len : usize, i : isize) -> usize {
    return  i.rem_euclid(len as isize) as usize;
}

fn mix(numbers: &Vec<isize>, times: usize) -> Vec<isize> {
    let length = numbers.len();
    let mut result = Vec::new();
    for i in 0..length {
        let n = numbers[i];
        result.push((n, i, false));
    }
    for mix_number in 0..times {
        for i in 0..length {
            let index = result.iter().position(|x| x.1 == i).unwrap();
            let item = result[index];
            let n = item.0 % (length - 1) as isize;
            if n == 0 {
                continue;
            }
            if n > 0 {
                // swap to move right n times
                let mut j = index;
                for _ in 0..n {
                    if j == length - 1 {
                        j = 0;
                        let last = result[length - 1];
                        result.remove(length - 1);
                        result.insert(0, last);
                    }
                    let current = get_index(length, j as isize);
                    let next = get_index(length, j as isize + 1);
                    result.swap(current, next);
                    j += 1;
                }
                if j == length - 1 {
                    let last = result[length - 1];
                    result.remove(length - 1);
                    result.insert(0, last);
                }
            }
            if n < 0 {
                // swap to move left n times
                let mut j = index as isize;
                for _ in 0..n.abs() {
                    if j == 0 {
                        j = length as isize - 1;
                        let first = result[0];
                        result.remove(0);
                        result.push(first);
                    }
                    let current = get_index(length, j as isize);
                    let next = get_index(length, j as isize - 1);
                    result.swap(current, next);
                    j -= 1;
                }
                if j == 0 {
                    let first = result[0];
                    result.remove(0);
                    result.push(first);
                }
            }
        }      
    }
    return result.iter().map(|x| x.0).collect();
}