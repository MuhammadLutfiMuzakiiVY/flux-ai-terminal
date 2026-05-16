# <img src="assets/images/logo.png" width="48" height="48" style="vertical-align:middle"> Flux AI Terminal (한국어 문서)
### *궁극의 네이티브 Rust 모바일 워크스테이션*

![Flux AI Terminal Banner](assets/images/banner.png)

---

## 🌍 글로벌 문서
- [English (Main)](README.md)
- [中文 (Chinese)](README.zh.md)
- [日本語 (Japanese)](README.jp.md)
- [한국어 (Korean)](README.kr.md)
- [العربية (Arabic)](README.ar.md)
- [Español (Spanish)](README.es.md)

---

## 🎯 프로젝트 비전
**Flux AI Terminal**은 단순한 에뮬레이터가 아닙니다. 이동 중인 개발자에게 권한을 부여하기 위해 처음부터 구축된 네이티브 실행 환경입니다. **Rust**의 성능과 **로컬 AI**의 지능을 활용하여, Flux는 주머니에 쏙 들어가는 지연 없는 보안 고성능 확장형 Linux 워크스테이션을 제공합니다.

### 🌟 왜 Flux인가요?
1. **모바일에서의 데스크톱 파워:** 컴파일러, 빌드 도구, 웹 서버를 네이티브로 실행.
2. **오프라인 지능:** 인터넷 연결 없이 AI를 실행하여 개인 정보를 보호.
3. **강화된 보안:** 소스 코드는 생체 인식 암호화 및 실시간 명령 방화벽에 의해 보호됩니다.

---

## 🏗️ 아키텍처 심층 분석

### 🦀 Rust 핵심 엔진
Flux의 핵심은 Rust로 구현된 비동기 비차단 커널입니다. `tokio` 런타임을 활용하여 최소한의 CPU 오버헤드로 수천 개의 동시 작업을 관리합니다.

#### 구성 요소:
- **PTY 에뮬레이션:** NeoVim 및 htop과 같은 복잡한 TUI 앱을 렌더링하기 위한 완전한 Xterm 호환 의사 터미널.
- **VFS (가상 파일 시스템):** 호스트 시스템을 수정하지 않고 전체 Ubuntu rootfs를 제공하는 OverlayFS 스타일의 샌드박스.
- **패키지 관리자:** 원자적 패키지 관리를 위한 `dpkg` 및 `apt`의 네이티브 구현.

### 🧠 AI RAG 엔진
Flux에는 로컬라이즈된 **검색 증강 생성 (RAG)** 엔진이 포함되어 있습니다. 로컬 문서와 man 페이지를 벡터 데이터베이스에 색인하여 AI가 오프라인에서 문맥에 맞는 정확한 제안을 제공할 수 있도록 합니다.

---

## 🛡️ 보안 백서

### 1. 암호화 엔클레이브
모든 민감한 토큰과 git 자격 증명은 **AES-256-GCM** 암호화된 저장소에 저장됩니다. 키는 장치의 하드웨어 엔클레이브(Secure Element)와의 생체 인식 핸드셰이크에 성공한 후에만 파생됩니다.

### 2. 정규식 명령 방화벽
모든 입력 문자열은 휴리스틱 방화벽에 의해 감사됩니다. `rm -rf /` 또는 무단 네트워크 액세스와 같은 파괴적인 패턴이 감지되면 명령이 차단되고 기록됩니다.

---

## 📅 로드맵 2026 - 2027

### 1단계: 안정성 (현재)
- [x] 멀티 아키텍처 지원 (ARM64, x86_64).
- [x] 네이티브 Apt/Dpkg.
- [x] 생체 인식 저장소.

### 2단계: GUI 및 GPU (2026 Q3)
- [ ] **Wayland 디스플레이 서버:** GUI 앱을 네이티브로 실행.
- [ ] **GPU 가속:** AI 추론을 위한 Vulkan 지원.

---

## 📄 라이선스
MIT 라이선스. Copyright (c) 2026 Flux AI Team.
