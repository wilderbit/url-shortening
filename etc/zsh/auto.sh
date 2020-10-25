export PYTHONPATH=${PROJDIR}/dj:${PROJDIR}/

function pushd2() {
  PUSHED="$(pwd)"
  cd "$PROJDIR""$1" >>/dev/null
}

function popd2() {
  cd "${PUSHED:-"$PROJDIR"}" >>/dev/null
  unset PUSHED
}

function manage() {
  pushd2 /dj
  python manage.py $*
  r=$?
  popd2
  return ${r}
}

function showmigrations() {
  manage showmigrations $*
}

function makemigrations() {
  manage makemigrations $*
}

function migrate() {
  manage migrate $*
}

function djshell() {
  manage shell
}

function dbshell() {
  manage dbshell
}

function createsuperuser() {
  manage createsuperuser
}

function recreatedb() {
  psql -h localhost -U postgres -c "CREATE USER root IF NOT EXISTS;"
  psql -h localhost -U postgres -c "ALTER USER root WITH SUPERUSER;"
  psql -h localhost -c "DROP DATABASE IF EXISTS url_shortening;" template1
  psql -h localhost -c "CREATE DATABASE url_shortening" template1
  migrate $*
}

function pyfmt() {
  black $PROJDIR/dj
}

function install_diesel() {
  cargo install diesel_cli --no-default-features --features "postgres"
}

function diesel_schema() {
  python $PROJDIR/dj/scripts/create_diesel_toml.py
  if [[ $? != 0 ]]; then
    echo "failure to update diesel schema"
    return $?
  fi
  (which diesel || install_diesel) >> /dev/null
  diesel print-schema only-tables > $PROJDIR/service/src/models/schema.rs
  sed -i '' -e 's/Varchar/Text/g' $PROJDIR/service/src/models/schema.rs
}