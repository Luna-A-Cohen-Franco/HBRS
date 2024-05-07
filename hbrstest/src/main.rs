use hbrs::utils::byte_arrays_helper::ByteArraysHelper;

fn main(){
    print!("{:?}", ByteArraysHelper::word_to_port16(ByteArraysHelper::port16_to_word(255, 255)));
}