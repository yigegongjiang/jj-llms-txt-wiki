> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# ultrareview로 버그 찾기

> /code-review ultra를 사용하여 클라우드에서 심층적인 다중 에이전트 코드 리뷰를 실행하여 병합 전에 버그를 찾고 검증합니다.

<Note>
  Ultrareview는 연구 미리보기 기능입니다. 기능, 가격 책정 및 가용성은 피드백에 따라 변경될 수 있습니다. 이제 명령어는 `/code-review ultra`로 호출되며, `/ultrareview`는 별칭으로 유지됩니다.
</Note>

Ultrareview는 Claude Code 웹 인프라에서 실행되는 심층 코드 리뷰입니다. `/code-review ultra`를 실행하면 Claude Code는 원격 샌드박스에서 리뷰어 에이전트 플릿을 시작하여 브랜치 또는 풀 요청의 버그를 찾습니다.

로컬 `/code-review` 또는 `/review`와 비교하여 ultrareview는 다음을 제공합니다:

* **더 높은 신호**: 보고된 모든 발견 사항이 독립적으로 재현되고 검증되므로 결과는 스타일 제안보다는 실제 버그에 초점을 맞춥니다
* **더 광범위한 범위**: 더 큰 리뷰어 에이전트 플릿이 변경 사항을 병렬로 탐색하므로 로컬 리뷰에서 놓칠 수 있는 문제를 표면화합니다
* **로컬 리소스 사용 없음**: 리뷰가 원격 샌드박스에서 완전히 실행되므로 실행 중에 터미널이 다른 작업을 위해 자유로워집니다

Ultrareview는 Claude Code 웹 인프라에서 실행되기 때문에 Claude.ai 계정으로 인증이 필요합니다. API 키만으로 로그인한 경우 `/login`을 실행하고 먼저 Claude.ai로 인증하세요. Ultrareview는 Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry와 함께 Claude Code를 사용할 때 사용할 수 없으며, Zero Data Retention을 활성화한 조직에서도 사용할 수 없습니다.

<h2 id="run-ultrareview-from-the-cli">
  CLI에서 ultrareview 실행
</h2>

Claude Code CLI의 모든 git 저장소에서 리뷰를 시작합니다.

```text theme={null}
/code-review ultra
```

인수 없이 ultrareview는 현재 브랜치와 기본 브랜치 간의 차이를 검토하며, 작업 트리의 커밋되지 않은 변경 사항과 스테이징된 변경 사항을 포함합니다. Claude Code는 저장소 상태를 번들로 묶고 리뷰를 위해 원격 샌드박스에 업로드합니다.

대신 GitHub 풀 요청을 검토하려면 PR 번호를 전달합니다.

```text theme={null}
/code-review ultra 1234
```

PR 모드에서 원격 샌드박스는 로컬 작업 트리를 번들로 묶는 대신 호스트에서 직접 풀 요청을 복제합니다. PR 모드는 `github.com`의 저장소 및 관리자가 Claude Code에 연결한 [GitHub Enterprise Server](/docs/ko/github-enterprise-server) 인스턴스에서 작동합니다.

<Tip>
  저장소가 너무 커서 번들로 묶을 수 없는 경우 Claude Code는 대신 PR 모드를 사용하도록 요청합니다. 브랜치를 푸시하고 초안 PR을 열고 `/code-review ultra <PR-number>`를 실행합니다.

  풀 요청의 차이가 너무 큰 경우 Claude Code는 리뷰 작업이 실행되기 전에 범위 지정 힌트와 함께 리뷰를 거부합니다.
</Tip>

시작하기 전에 Claude Code는 리뷰 범위(브랜치를 검토할 때 파일 및 라인 수 포함), 남은 무료 실행 횟수 및 예상 비용이 포함된 확인 대화를 표시합니다. 확인한 후 리뷰는 백그라운드에서 계속되며 세션을 계속 사용할 수 있습니다. 명령은 `/code-review ultra`로 호출할 때만 실행되며, Claude는 자동으로 ultrareview를 시작하지 않습니다.

<h2 id="pricing-and-free-runs">
  가격 책정 및 무료 실행
</h2>

Ultrareview는 플랜의 포함된 사용량이 아닌 사용량 크레딧에 대해 청구되는 프리미엄 기능입니다.

| 플랜                | 포함된 무료 실행 | 무료 실행 후                                                                                            |
| ----------------- | --------- | -------------------------------------------------------------------------------------------------- |
| Pro               | 3회 무료 실행  | [사용량 크레딧](https://support.claude.com/ko/articles/12429409-extra-usage-for-paid-claude-plans)으로 청구됨 |
| Max               | 3회 무료 실행  | [사용량 크레딧](https://support.claude.com/ko/articles/12429409-extra-usage-for-paid-claude-plans)으로 청구됨 |
| Team 및 Enterprise | 없음        | [사용량 크레딧](https://support.claude.com/ko/articles/12429409-extra-usage-for-paid-claude-plans)으로 청구됨 |

Pro 및 Max 구독자는 기능을 시도하기 위해 3회의 무료 ultrareview 실행을 받습니다. 이 3회 실행은 계정당 일회성 할당이며 새로 고쳐지지 않습니다. 3회를 모두 사용한 후 또는 무료 실행 기간이 끝난 후 각 리뷰는 사용량 크레딧으로 청구되며 일반적으로 변경 사항의 크기에 따라 $5에서 $20의 비용이 듭니다. 원격 세션이 시작되면 실행이 계산되므로 조기에 중단하거나 완료되지 않은 리뷰도 무료 실행을 사용합니다. 유료 리뷰의 경우 사용량 크레딧은 실행된 부분에 대해서만 청구됩니다.

Ultrareview는 항상 무료 실행 외에 사용량 크레딧으로 청구되기 때문에 계정 또는 조직은 유료 리뷰를 시작하기 전에 사용량 크레딧을 활성화해야 합니다. 사용량 크레딧이 활성화되지 않은 경우 Claude Code는 시작을 차단하고 켤 수 있는 청구 설정으로 연결합니다. `/usage-credits`를 실행하여 현재 설정을 확인하거나 변경할 수도 있습니다.

<h2 id="track-a-running-review">
  실행 중인 리뷰 추적
</h2>

리뷰는 일반적으로 5\~10분이 소요됩니다. 리뷰는 백그라운드 작업으로 실행되므로 세션에서 계속 작업하고, 다른 명령을 시작하거나, 터미널을 완전히 닫을 수 있습니다.

`/tasks`를 사용하여 실행 중이고 완료된 리뷰를 보고, 리뷰의 상세 보기를 열거나, 진행 중인 리뷰를 중지합니다. 리뷰를 중지하면 클라우드 세션이 보관되고 부분 발견 사항은 반환되지 않습니다. 리뷰가 완료되면 검증된 발견 사항이 세션의 알림으로 나타납니다. 각 발견 사항에는 파일 위치와 문제에 대한 설명이 포함되어 있으므로 Claude에 직접 수정을 요청할 수 있습니다.

<h2 id="run-ultrareview-non-interactively">
  비대화형으로 ultrareview 실행
</h2>

`claude ultrareview` 하위 명령을 사용하여 대화형 세션 없이 CI 또는 스크립트에서 ultrareview를 시작합니다. 하위 명령은 `/code-review ultra`와 동일한 리뷰를 시작하고, 원격 리뷰가 완료될 때까지 차단하고, 발견 사항을 stdout으로 인쇄하고, 성공 시 코드 0으로 종료하거나 실패 시 코드 1로 종료합니다.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

인수 없이 하위 명령은 현재 브랜치와 기본 브랜치 간의 차이를 검토합니다. PR 번호를 전달하여 풀 요청을 검토하거나, 기본 브랜치를 전달하여 대신 해당 브랜치에 대한 차이를 검토합니다. 하위 명령을 호출하면 대화형 명령이 표시하는 청구 및 약관 프롬프트에 대한 동의로 간주됩니다.

진행 메시지 및 라이브 세션 URL은 stderr로 이동하므로 stdout은 구문 분석 가능한 상태로 유지됩니다. 이 플래그를 사용하여 출력 및 시간 초과를 제어합니다:

| 플래그                   | 설명                                          |
| --------------------- | ------------------------------------------- |
| `--json`              | 형식이 지정된 발견 사항 대신 원본 `bugs.json` 페이로드를 인쇄합니다 |
| `--timeout <minutes>` | 리뷰가 완료될 때까지 기다릴 최대 분 수입니다. 기본값은 30입니다       |

`claude ultrareview` 실행은 `/code-review ultra`와 동일한 인증 및 사용량 크레딧 구성이 필요합니다. 하위 명령은 리뷰가 발견 사항 유무와 관계없이 완료될 때 코드 0으로 종료되고, 리뷰가 시작되지 않거나 원격 세션에 오류가 발생하거나 시간 초과가 경과할 때 코드 1로 종료되며, Ctrl-C로 중단될 때 코드 130으로 종료됩니다. 하위 명령을 중단하면 원격 리뷰는 계속 실행됩니다. stderr로 인쇄된 세션 URL을 따라 브라우저에서 시청합니다.

GitHub 풀 요청에 대한 자동 리뷰의 경우, [Code Review](/docs/ko/code-review)는 저장소와 직접 통합되고 CLI 단계 없이 발견 사항을 인라인 PR 댓글로 게시합니다.

<h2 id="how-ultrareview-compares-to-/code-review-and-/review">
  ultrareview와 /code-review 및 /review의 비교
</h2>

세 명령 모두 코드를 검토하지만 워크플로우의 다른 단계를 대상으로 합니다.

|       | `/code-review` | `/review <pr>`       | `/code-review ultra`         |
| ----- | -------------- | -------------------- | ---------------------------- |
| 대상    | 작업 중인 diff     | GitHub 풀 요청          | 작업 중인 diff 또는 풀 요청           |
| 실행    | 세션에서 로컬로       | 세션에서 로컬로             | 클라우드 샌드박스에서 원격으로             |
| 깊이    | 노력 인수에 따라 확장   | 세션의 노력 수준에서 단일 패스 검토 | 독립적 검증이 있는 다중 에이전트 플릿        |
| 기간    | 몇 초에서 몇 분      | 몇 초에서 몇 분            | 대략 5\~10분                    |
| 비용    | 일반 사용량으로 계산됨   | 일반 사용량으로 계산됨         | 무료 실행, 그 후 대략 $5~$20 사용량 크레딧 |
| 최적 용도 | 반복하는 동안 빠른 피드백 | 승인하기 전에 팀원의 PR 검토    | 실질적인 변경 사항을 병합하기 전에 신뢰도 향상   |

작업하는 동안 빠른 피드백을 위해 `/code-review`를 사용합니다. 풀 요청을 승인하기 전에 검토하는 방식으로 살펴보려면 `/review <pr>`을 사용합니다. 로컬 검토에서 놓칠 수 있는 문제를 포착하는 더 깊은 패스를 원할 때 실질적인 변경 사항을 병합하기 전에 `/code-review ultra`를 사용합니다.

<h2 id="related-resources">
  관련 리소스
</h2>

* [Claude Code on the web](/docs/ko/claude-code-on-the-web): 클라우드 세션 및 클라우드 샌드박스의 작동 방식 알아보기
* [ultraplan으로 복잡한 변경 사항 계획](/docs/ko/ultraplan): 사전 설계 작업을 위한 ultrareview의 계획 대응
* [비용 효과적으로 관리](/docs/ko/costs): 사용량 추적 및 지출 한도 설정
