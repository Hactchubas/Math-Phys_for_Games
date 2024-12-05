#[cfg(test)]
mod tests {
    use crate::structs::binary::BinaryFactory;

    #[test]
    fn test_create_valid_binary() {
        let binary_str = "101010";
        let binary_obj = BinaryFactory::create(binary_str);

        assert!(binary_obj.is_some(), "Expected Binary object to be created");
        let binary = binary_obj.unwrap();
        assert_eq!(binary.get_binary(), "101010");
    }

    #[test]
    fn test_create_invalid_binary() {
        let binary_str = "1023"; // caracteres inválidos
        let binary_obj = BinaryFactory::create(binary_str);

        assert!(
            binary_obj.is_none(),
            "Expected None for invalid binary string"
        );
    }

    #[test]
    fn test_next_binary() {
        let binary_str = "0";
        let binary_obj = BinaryFactory::create(binary_str).unwrap();

        let next_binary = binary_obj.next();
        assert_eq!(next_binary.get_binary(), "1");
    }
   

    #[test]
    fn test_two_complement() {
        let binary_str = [
            ("-0001", "1111"),
            ("1001", "-0111"),
            ("-011", "101"),
            ("101", "-011"),
            ("-1111", "0001"),
            ("1001", "-0111"),
            ("0", "0"),
            ("1", "-1"),
        ];
        for (x, y) in binary_str {
            let binary_obj = BinaryFactory::create(&x).unwrap();

            let two_complement_binary = binary_obj.two_complement();
            assert_eq!(two_complement_binary.get_binary(), y);
        }
    }

    #[test]
    fn test_invert_binary() {
        let binary_str = "0101";
        let binary_obj = BinaryFactory::create(binary_str).unwrap();

        let inverted_binary = binary_obj.invert();
        assert_eq!(inverted_binary.get_binary(), "1010"); // Esperado: "1010"
    }

    #[test]
    fn test_bitwise_and() {
        let binary_str1 = "1010";
        let binary_str2 = "1100";
        let binary_obj1 = BinaryFactory::create(binary_str1).unwrap();
        let binary_obj2 = BinaryFactory::create(binary_str2).unwrap();

        let result = binary_obj1 & binary_obj2;
        assert_eq!(result.get_binary(), "1000"); // Esperado: "1000"
    }

    #[test]
    fn test_bitwise_or() {
        let binary_str1 = "1010";
        let binary_str2 = "1100";
        let binary_obj1 = BinaryFactory::create(binary_str1).unwrap();
        let binary_obj2 = BinaryFactory::create(binary_str2).unwrap();

        let result = binary_obj1 | binary_obj2;
        assert_eq!(result.get_binary(), "1110"); // Esperado: "1110"
    }

    #[test]
    fn test_bitwise_xor() {
        let binary_str1 = "1010";
        let binary_str2 = "1100";
        let binary_obj1 = BinaryFactory::create(binary_str1).unwrap();
        let binary_obj2 = BinaryFactory::create(binary_str2).unwrap();

        let result = binary_obj1 ^ binary_obj2;
        assert_eq!(result.get_binary(), "0110"); // Esperado: "0110"
    }

    #[test]
    fn test_add_binary() {
        let binary_str = [
            ("1", "1", "10"),
            ("11", "1111", "10010"),
            ("111", "101", "1100"),
            ("111", "-11", "100"),
            ("-0001", "1111", "1110"),
            ("1001", "-0111", "0010"),
            ("-011", "101", "010"),
            ("101", "-011", "010"),
            ("-1111", "0001", "-1110"),
            ("-1001", "-0111", "-10000"),
            ("0", "0", "0"),
            ("1", "-1", "0"),
            ("101", "-011", "010"),
        ];
        for (x, y, z) in binary_str {
            let binary_obj = BinaryFactory::create(&x).unwrap();
            let binary_obj2 = BinaryFactory::create(&y).unwrap();
            
            let sum = binary_obj + binary_obj2;
            assert_eq!(sum.get_binary(), z);
        }
    }
    
    #[test]
    fn test_sub_binary() {
        let binary_str = [
            ("1", "1", "0"),
            ("11", "1111", "-1100"),
            ("111", "101", "010"),
            ("111", "-11", "1010"),
            ("-0001", "1111", "-10000"),
            ("1001", "-0111", "10000"),
            ("-011", "101", "-1000"),
            ("101", "-011", "1000"),
            ("-1111", "0001", "-10000"),
            ("-1001", "-0111", "-0010"),
            ("0", "0", "0"),
            ("1", "-1", "10"),
            ("101", "1", "100"),
            ("101", "11", "010"),
        ];
        for (x, y, z) in binary_str {
            let binary_obj = BinaryFactory::create(&x).unwrap();
            let binary_obj2 = BinaryFactory::create(&y).unwrap();
            
            let sum = binary_obj - binary_obj2;
            assert_eq!(sum.get_binary(), z);
        }
    }
    
    
    
    #[test]
    fn test_mul_binary() {
        let binary_str = [
        ("10", "11", "110"),    
        ("10", "10", "100"),    
        ("0", "10", "0"),       
        ("10", "0", "0"),       
        ("11", "11", "1001"),   
        ("10", "-11", "-110"),  
        ("-10", "-11", "110"),  
        ("-10", "11", "-110"),  
        ("10", "101", "1010"),  
    ];
        for (x, y, z) in binary_str {
            let binary_obj = BinaryFactory::create(&x).unwrap();
            let binary_obj2 = BinaryFactory::create(&y).unwrap();
            
            let sum = binary_obj * binary_obj2;
            assert_eq!(sum.get_binary(), z);
        }
    }

}
