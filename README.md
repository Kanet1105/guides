# GUIDES

## Single-threaded Runtime (addressbook)
#### Core
##### 1. default.rs
- 커스텀 타입 aliasing 및 초기화 기능 제공 모듈.

##### 2. runtime.rs
- 콜백 타입으로 등록한 유저 함수들을 실행 가능하게 해주는 런타임.
- Rc 와 RefCell 로 필드를 감싼 형태라서 single thread 에서만 안전.

#### Components
##### 1. menu.rs
- 사용자 정의 함수 컴포넌트들이 들어감 Rc 로 함수 포인터 감싼 형태의
Callback 타입으로 초기화된 후 프로그램에서 실행되는 함수들. 프로그램
기본 빌딩 블럭.

## Multi-threaded Runtime (http)
