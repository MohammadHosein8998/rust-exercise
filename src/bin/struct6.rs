struct ShortDuration(u32 , u32);


struct LongDuration(u32 , u32);

fn main(){
    let work_shift =ShortDuration(8 ,17);
    go_to_work(work_shift)
}


fn go_to_work(time : ShortDuration){
    println!("time to go work : {} to {}",time.0 , time.1);
}