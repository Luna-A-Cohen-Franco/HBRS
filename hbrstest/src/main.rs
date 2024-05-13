use hbrs::utils::float_helper::FloatHelper;

fn main(){
    let test = vec![1,2,3,3,4];
    let mut vac = vec![];
    print!("{:?}", vac.copy_from_slice(&test[..]));
}