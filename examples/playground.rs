// Complete the function to make the program execute successfully.

fn remove_if_odd(nums: &mut Vec<i32>, index: usize) {
    if index > nums.len() {
        println!("Index out of bounds");
        return;
    } else {

        if let Some(num) = nums.get(index) {
            if num % 2 == 0 {
                println!("Is Even");
            }
            else {
                println!("Is Odd");
                nums.remove(index);
            }

        }

    }
}

fn main() {
    let mut nums = vec![1, 2, 6, 9];
    let nums_ref = &mut nums;
    remove_if_odd(nums_ref, 0);
    remove_if_odd(nums_ref, 1);
    remove_if_odd(nums_ref, nums_ref.len() - 1);
    // assert_eq!(nums.len(), 2);
    println!("{:?}", nums);


    println!("XXXXXXXXXXXXXxx");
    let mut aaa = "xxx".to_string();
    let asd = &mut aaa;
    asd.push_str("vbnvbn");
    
    println!("aaa:{}", aaa);
    println!("asd:{}", asd); 

}
