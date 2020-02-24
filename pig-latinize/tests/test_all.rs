use pig_latinize::pig_latinize;
use std::string::String;

#[test]
fn test_foo_bar_baz() {
    {
        let result = pig_latinize(String::from("foo bar baz"));

        assert_eq!(result, "oofay arbay azbay");
    }
}

#[test]
fn test_hello_world() {
    {
        let result = pig_latinize(String::from("hello world"));

        assert_eq!(result, "ellohay orldway");
    }
}

#[test]
fn test_captain_america() {
    {
        let result = pig_latinize(String::from("captain america"));

        assert_eq!(result, "aptaincay americahay");
    }
}

#[test]
fn test_iron_man() {
    {
        let result = pig_latinize(String::from("iron man"));

        assert_eq!(result, "ironhay anmay");
    }
}

#[test]
fn test_scarlet_witch() {
    {
        let result = pig_latinize(String::from("scarlet witch"));

        assert_eq!(result, "arletscay itchway");
    }
}

#[test]
fn test_yondu_udonta() {
    {
        let result = pig_latinize(String::from("yondu udonta"));

        assert_eq!(result, "onduyay udontahay");
    }
}

#[test]
fn test_anakin_skywalker() {
    {
        let result = pig_latinize(String::from("anakin skywalker"));

        assert_eq!(result, "anakinhay ywalkerskay");
    }
}
