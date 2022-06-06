use std::os::raw::c_long;

#[derive(Debug, Clone, Copy)]
enum PokerSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug)]
enum PokerCardWithValue {
    Clubs(u8),
    Spades(u8),
    Hearts(u8),
    Diamonds(u8),
}

// 任何类型的数据都可以放入枚举成员中: 例如字符串、数值、结构体甚至另一个枚举。
#[derive(Debug)]
enum Message {
    Quit,
    Move {x :i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

//  Rust中如何处理null

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    

    struct PokerCard {
        card: PokerSuit,
        value: u32
    }

    // 枚举值通过::来访问 
    let heart = PokerSuit::Hearts;
    let poker_card = PokerCard {
        card: heart,
        value: 1
    };

    let msgs = [
        Message::Quit
    ];

    let heart_with_value = PokerCardWithValue::Hearts(9);
    let mut data1: Option<i32> = None;
    let data2 = 32;
    data1 = Some(24);
    println!("{}", data1.unwrap() + data2);
    handle_data(Message::Quit);
    println!("{:?}", heart_with_value);
    print_suit(heart);
    print_suit(poker_card.card);
    let val = times_some_value(Some(3));
    println!("{}", val.unwrap());




    // 数组 

    // 固定长度的数组
    // 数据类型一致
    let mut arr = [1, 2, 3];

    // 通过索引访问
    println!(" arr[1] is {}", arr[1]);

    // 和字符串类似，数组也能访问切片

    let slice = &arr[0..3];

    println!("{:?}", slice);
    arr[2] = 110;
    println!("{:?}", arr);

}

fn print_suit(card: PokerSuit){
    println!("{:?}", card);
}

// 由于每个结构体都有自己的类型，因此我们无法在需要同一类型的地方进行使用，例如某个函数它的功能是接受消息并进行发送，那么用枚举的方式，就可以接收不同的消息，但是用结构体，该函数无法接受 4 个不同的结构体作为参数。
fn handle_data(value: Message){
    println!("{:?}", value);
}

fn times_some_value(x: Option<i32>)-> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i * 2)
    }
}
