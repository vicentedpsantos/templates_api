#### List templates
```bash
curl 127.0.0.1:8000/templates
```
#### Create new templates
```bash
curl 127.0.0.1:8000/templates -X POST -d '{"name": "welcome-template", "content":"Hello\nThis is an email template\n"}' -H "Content-type: application/json"
```
