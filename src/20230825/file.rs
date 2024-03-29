//!한 번에 한 단계씩 파일을 시뮬레이트 한다.

///아마도 파일 시스템에 있을
/// "파일"을 나타낸다.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    /// 새 파일은 비어있다고 가정하지만 이름은 필요하다.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    /// 파일 길이를 바이트로 반환하다.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 파일 이름을 반환한다.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
