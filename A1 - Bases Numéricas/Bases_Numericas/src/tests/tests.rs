#[cfg(test)]
mod tests {
    use crate::structs::binary::Binary;

    #[test]
    fn test_next_binary() {
        let mut binary_test = Binary::new("0");
        assert_eq!(binary_test.next().value, "1");

        binary_test = Binary::new("1");
        assert_eq!(binary_test.next().value, "10");

        binary_test = Binary::new("101");
        assert_eq!(binary_test.next().value, "110");

        binary_test = Binary::new("111");
        assert_eq!(binary_test.next().value, "1000");

    }

    #[test]
    fn test_inverse_binary(){
        let mut binary_test = Binary::new("0");
        assert_eq!(binary_test.invert().value, "1");

        binary_test = Binary::new("1");
        assert_eq!(binary_test.invert().value, "0");

        binary_test = Binary::new("101");
        assert_eq!(binary_test.invert().value, "010");

        binary_test = Binary::new("111");
        assert_eq!(binary_test.invert().value, "000");
    }

    #[test]
    fn test_bitwise_and_binary(){
        let mut binary_test1 = Binary::new("1010");
        let mut binary_test2 = Binary::new("1100");
        let mut test_result= binary_test1 & binary_test2;
        assert_eq!( test_result.value, "1000");
        
        
        binary_test1 = Binary::new("1111");
        binary_test2 = Binary::new("0000");
        test_result = binary_test1 & binary_test2;
        assert_eq!( test_result.value, "0000");
        
        
        binary_test1 = Binary::new("101");
        binary_test2 = Binary::new("110");
        test_result= binary_test1 & binary_test2;
        assert_eq!( test_result.value, "100");           
        
        
        binary_test1 = Binary::new("1011");
        binary_test2 = Binary::new("111");
        test_result= binary_test1 & binary_test2;
        assert_eq!( test_result.value, "0011");           
        
    }
    
    #[test]
    fn test_bitwise_or_binary(){
        let mut binary_test1 = Binary::new("1010");
        let mut binary_test2 = Binary::new("1100");
        let mut test_result= binary_test1 | binary_test2;
        assert_eq!( test_result.value, "1110");
        
        
        binary_test1 = Binary::new("1111");
        binary_test2 = Binary::new("0000");
        test_result = binary_test1 | binary_test2;
        assert_eq!( test_result.value, "1111");
        
        
        binary_test1 = Binary::new("101");
        binary_test2 = Binary::new("110");
        test_result= binary_test1 | binary_test2;
        assert_eq!( test_result.value, "111");           
        

        binary_test1 = Binary::new("101010");
        binary_test2 = Binary::new("1100");
        test_result= binary_test1 | binary_test2;
        assert_eq!( test_result.value, "101110");           
        
    }
   
   
   
    #[test]
    fn test_bitwise_xor_binary(){
        let mut binary_test1 = Binary::new("1010");
        let mut binary_test2 = Binary::new("1100");
        let mut test_result= binary_test1 ^ binary_test2;
        assert_eq!( test_result.value, "0110");
        
        
        binary_test1 = Binary::new("1111");
        binary_test2 = Binary::new("0000");
        test_result = binary_test1 ^ binary_test2;
        assert_eq!( test_result.value, "1111");
        
        
        binary_test1 = Binary::new("101");
        binary_test2 = Binary::new("110");
        test_result= binary_test1 ^ binary_test2;
        assert_eq!( test_result.value, "011");           
        

        binary_test1 = Binary::new("101010");
        binary_test2 = Binary::new("1100");
        test_result= binary_test1 ^ binary_test2;
        assert_eq!( test_result.value, "100110");           
        
    }
    
    
    #[test]
    fn test_add_binary(){
        let mut binary_test1 = Binary::new("0");
        let mut binary_test2 = Binary::new("0");
        let mut test_result= binary_test1 + binary_test2;
        assert_eq!( test_result.value, "0");
        
        
        binary_test1 = Binary::new("1");
        binary_test2 = Binary::new("1");
        test_result = binary_test1 + binary_test2;
        assert_eq!( test_result.value, "10");
        
        
        binary_test1 = Binary::new("101");
        binary_test2 = Binary::new("110");
        test_result= binary_test1 + binary_test2;
        assert_eq!( test_result.value, "1011");           
        

        binary_test1 = Binary::new("1111");
        binary_test2 = Binary::new("1");
        test_result= binary_test1 + binary_test2;
        assert_eq!( test_result.value, "10000");           
        
    }
}
