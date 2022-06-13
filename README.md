# Function
User object can be stored and retrieved when running with local redis server.

## Requests

---
### GET http://localhost:8000/health

**Response** - 200 OK "Application Healthy"

---

### POST http://localhost:8000/user

**Request**
```json
{
	"id": "42069",
	"name": "WonkyMic"
}
```
**Response** - 202 Accepted "upsert user"

---

### GET http://localhost:8000/user/<id>

**Response** - 200 OK "Hello, `<name>`!"

---

# Web Stack
- Cache :: Redis
- Database :: Undecided
-- Cassandra
-- Web3 Solution
- Framework :: Rocket

# Notes
### Connection Pooling
We'll use [rocket_contrib::databases](https://api.rocket.rs/v0.4/rocket_contrib/databases/index.html) to connect to our Redis cache.

