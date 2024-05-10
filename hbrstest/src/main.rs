use hbrs::utils::float_helper::FloatHelper;

fn main(){
    let value = 3.14;
    let result = FloatHelper::double_to_string(value, 0);
    println!("Value: {}", result);
}