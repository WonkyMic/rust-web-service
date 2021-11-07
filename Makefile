.PHONY : build run make dbconfig
make :
	cargo run
build :
	docker build . -t wonkymic/wonkyapi
run :
	docker run -p 8000:8000 wonkymic/wonkyapi
dbconfig : 
	psql -U wonkymic wonkydb