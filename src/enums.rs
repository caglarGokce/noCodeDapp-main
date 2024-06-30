#[derive(Clone)]
pub enum TheOperation{
  Addition{subject_1:TheSubjects,subject_2:TheSubjects,result:TheSubjects},//100
  Substraction{subject_1:TheSubjects,subject_2:TheSubjects,result:TheSubjects},//200
  Multiplication{subject_1:TheSubjects,subject_2:TheSubjects,result:TheSubjects},//300
  Division{subject_1:TheSubjects,subject_2:TheSubjects,result:TheSubjects},//400
}//4

#[derive(Clone)]
pub enum TheConditions{
  IsEqual{compare_1:TheSubjects,compare_2:TheSubjects,operation:TheOperation},//1
  IsEqualNot{compare_1:TheSubjects,compare_2:TheSubjects,operation:TheOperation},//2
  IsBigger{compare_1:TheSubjects,compare_2:TheSubjects,operation:TheOperation},//3
  IsSmaller{compare_1:TheSubjects,compare_2:TheSubjects,operation:TheOperation},//5
  IsBiggerEqual{compare_1:TheSubjects,compare_2:TheSubjects,operation:TheOperation},//7
  IsSmallerEqual{compare_1:TheSubjects,compare_2:TheSubjects,operation:TheOperation},//9
}

#[derive(Clone)]
pub enum TheSubjects{
  Outputs{no:u16},//1_000
  Inputs{no:u16},//2_000
  Constants{no:u16},//3_000
  Fields{no:u16},//4_000
}

#[derive(Clone)]
pub enum ExeOrder{
  WithCondition{condition:TheConditions},
  WithoutCondition{operation:TheOperation},
}