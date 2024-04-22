/// computes the greatest common divisor of two integers, using Euclid’s algorithm
pub fn gcd(mut number1: u64, mut number2: u64) -> u64 {
    // Проверяем, что оба числа не равны 0
    assert!(number1 != 0 && number2 != 0);
    

    // Если второе число меньше первого, меняем их местами
    if number2 < number1 {
        let temp = number2;
        number2 = number1;
        number1 = temp;
    }
    // Применяем алгоритм Евклида, пока одно из чисел не станет равным 0
    while number2 != 0 {    

        if number2 < number1 {
            let temp = number2;
            number2 = number1;
            number1 = temp;
        }
        // Вычисляем остаток от деления второго числа на первое
        number2 = number2 % number1;
    }
    // Возвращаем результат - наибольший общий делитель
    number1
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(14, 15), 1);

        assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);

         // Тесты для различных пар чисел
         assert_eq!(gcd(42, 56), 14);
         assert_eq!(gcd(799459, 28823), 27347);
         
         assert_eq!(gcd(14, 15), 1);
         assert_eq!(gcd(24, 36), 12);
         assert_eq!(gcd(17, 34), 17);
         assert_eq!(gcd(8, 12), 4);
         assert_eq!(gcd(100, 75), 25);
         assert_eq!(gcd(54, 24), 6);
         assert_eq!(gcd(81, 27), 27);
         assert_eq!(gcd(48, 18), 6);
         assert_eq!(gcd(99, 88), 11);
         assert_eq!(gcd(120, 0), 120); // Тест с одним из чисел равным 0
         assert_eq!(gcd(0, 0), 0);     // Тест с обоими числами равными 0
    }
}
