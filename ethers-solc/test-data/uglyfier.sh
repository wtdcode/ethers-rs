#/bin/bash!

echo "uglyfying..."

for file in $(find . -name '*.json'); do
  ugly=$(jq -c '.' $file)
  echo $ugly > $file
done

echo "done!"
