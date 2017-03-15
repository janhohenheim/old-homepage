git pull
./scripts/package.sh
scp rpm/x86_64/millionaire-0.3.0-1.x86_64.rpm jnferner.com:~/
ssh jnferner.com rpm -Uhv millionaire-0.3.0-1.x86_64.rpm

