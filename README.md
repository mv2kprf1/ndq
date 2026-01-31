# NDQ: Minimal Necessary Data Quality Test Executor

`Still being worked on and not ready for use.`

## Design

Two core primitives:
- Only one true 'data quality check': user-defined SQL queries
  - Defined inline or through reusable SQL files
- Primacy of `count()`: what matters is "Did this test pass or fail, and did it find an anomalous count of rows?"

Text primacy for output:
- On run, allow either user-friendly text output or structured JSON output
- Allows for consumption by downstream tools: workflow orchestrators, Slack/PagerDuty alerting, etc.

Nothing more: no lock-in, no framework, nothing more than what is necessary.

## Composability

Slack notifications:
```bash
if ! ndq prod.yaml; then
    ndq prod.yaml --format json | \
      jq -r '.checks[] | select(.status=="fail") | .check_name' | \
      post-to-slack.sh
fi
```

Airflow workflows:
```python
BashOperator(
    task_id='data_quality',
    bash_command='ndq {{ params.config }}',
)
```

Parse results in any language:
```javascript
const result = JSON.parse(
    execSync('ndq test.yaml --format json').toString()
);
if (result.summary.failed > 0) {
    console.error('Quality checks failed');
    process.exit(1);
}
```

Installation:
```
cargo install ndq
```

License: AGPL
