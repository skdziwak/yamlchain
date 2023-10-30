#!/usr/bin/env bash

mkdir -p .git/hooks

cat <<EOF > .git/hooks/pre-commit
#!/usr/bin/env bash

cargo run -- --schema workflows-schema.json
if [ $? -ne 0 ]; then
  echo "Cargo run failed, aborting commit."
  exit 1
fi
git add workflows-schema.json
EOF

chmod +x .git/hooks/*
