mod test {
    use itertools::assert_equal;
    use teo_teon::Value;

    #[test]
    fn value_ref_into_vec_str() {
        let value = Value::Array(vec![Value::String("1".to_owned()), Value::String("2".to_owned())]);
        let value_ref = &value;
        let vec_str: Vec<&str> = value_ref.try_into().unwrap();
        assert_equal(vec_str, vec!["1", "2"])
    }

    #[test]
    fn value_ref_into_vec_string() {
        let value = Value::Array(vec![Value::String("1".to_owned()), Value::String("2".to_owned())]);
        let value_ref = &value;
        let vec_str: Vec<String> = value_ref.try_into().unwrap();
        assert_equal(vec_str, vec!["1".to_owned(), "2".to_owned()])
    }

    #[test]
    fn value_ref_into_vec_i32() {
        let value = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        let value_ref = &value;
        let vec_str: Vec<i32> = value_ref.try_into().unwrap();
        assert_equal(vec_str, vec![1, 2])
    }

    #[test]
    fn value_into_vec_i32() {
        let value = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        let vec_str: Vec<i32> = (&value).try_into().unwrap();
        assert_equal(vec_str, vec![1, 2])
    }
}
