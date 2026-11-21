use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("🎯 Добро пожаловать в игру 'Угадай число'!");
    println!("🎲 Я загадал число от 1 до 100. Попробуй угадать!");
    println!("💡 Введи 'quit' или 'выход', чтобы выйти из игры\n");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    loop {
        println!("🔢 Введи свою догадку:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Не удалось прочитать строку");
        let guess = guess.trim();
        if guess.eq_ignore_ascii_case("quit") || guess.eq_ignore_ascii_case("выход") {
            println!("👋 До свидания! Было приятно играть!");
            break;
        }
        let guess: u32 = match guess.parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("⚠️  Пожалуйста, введи число от 1 до 100!");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("❌ Это не число! Попробуй еще раз.");
                continue;
            }
        };
        attempts += 1;
        println!("🎯 Ты угадал: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📈 Слишком маленькое! Попробуй больше."),
            Ordering::Greater => println!("📉 Слишком большое! Попробуй меньше."),
            Ordering::Equal => {
                println!("🎉 Поздравляю! Ты угадал число {} за {} попыток!",
                         secret_number, attempts);
                match attempts {
                    1 => println!("🏆 Невероятно! Угадал с первого раза!"),
                    2..=5 => println!("⭐ Отлично! Очень быстро!"),
                    6..=10 => println!("👍 Хорошо! Неплохой результат!"),
                    _ => println!("😊 Главное, что угадал! Попробуй еще раз!")
                }
                println!("\n🔄 Хочешь сыграть еще раз? (да/нет)");
                let mut play_again = String::new();
                io::stdin()
                    .read_line(&mut play_again)
                    .expect("Не удалось прочитать строку");
                let play_again = play_again.trim().to_lowercase();
                if play_again == "да" || play_again == "yes" || play_again == "y" {
                    let secret_number = rand::thread_rng().gen_range(1..=100);
                    attempts = 0;
                    println!("\n🎲 Отлично! Я загадал новое число от 1 до 100!");
                    continue;
                } else {
                    println!("👋 Спасибо за игру! До встречи!");
                    break;
                }
            }
        }
        println!();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_generation() {
        for _ in 0..100 {
            let num = rand::thread_rng().gen_range(1..=100);
            assert!(num >= 1 && num <= 100);
        }
    }
    #[test]
    fn test_string_parsing() {
        assert_eq!("42".parse::<u32>(), Ok(42));
        assert!("abc".parse::<u32>().is_err());
        assert!("0".parse::<u32>(), Ok(0));
        assert!("101".parse::<u32>(), Ok(101));
    }
}