fn main() {
    let mut count=1;
    let mut sum=0;
    let target = 'd';
    for char in 'a'..='z' {
        sum = sum + char as i32;
        let avg = sum /count;

        if char == target {
            println!("{}:{}", char, char::from_u32(avg as u32).unwrap());
        }else{
            println!("{}:{}", char, avg);
        }
        count+=1;

    }
    println!("FP way");
    ('a'..='z').enumerate().fold(0, |acc, (i, x)| {
        let acc = (acc + x as i32);
        let avg = acc / ((i + 1) as i32);
        match x {
            'd' => println!("{}:{}", x, char::from_u32(avg as u32).unwrap()),
            _ => println!("{}:{}", x, avg),
        }
        acc
    });
}
