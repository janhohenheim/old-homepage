git pull
rm -f rpm/millionaire-*.rpm
./scripts/package.sh
ssh jnferner.com rm -f millionaire-*.rpm
scp rpm/x86_64/millionaire-*.rpm jnferner.com:~/
ssh jnferner.com rpm -Uhv millionaire-*.rpm

