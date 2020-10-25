"""
This scripts helps to select django models by app name
and generate diesel.toml file.
"""

import os
import django

os.environ["DJANGO_SETTINGS_MODULE"] = "proj.settings"
django.setup()
from django.db import connection
from django.conf import settings


"""
Need both include and exclude apps, to check no app has been missed 
"""
exclude_apps = [
    "django.contrib.admin",
    "django.contrib.auth",
    "django.contrib.contenttypes",
    "django.contrib.sessions",
    "django.contrib.messages",
    "django.contrib.staticfiles",
]

include_apps = ["url_shortening"]


def select_tables():
    if not len(include_apps):
        print("Include model app should not be empty")
        exit(1)

    include_set = set(include_apps)
    exclude_set = set(exclude_apps)

    intersect = include_set.intersection(exclude_set)
    union_set = include_set.union(exclude_set)
    installed_apps = set(settings.INSTALLED_APPS)

    if len(intersect):
        print("both contains similar names", intersect)
        exit(1)

    if union_set != installed_apps:
        if len(installed_apps - union_set):
            print("installed app name missing", installed_apps - union_set)
            exit(1)
        if len(union_set - installed_apps):
            print("App name is not a django app", union_set - installed_apps)
            exit(1)

    include_set_iter = list(map(lambda x: x + "_%", include_set))
    like_query = "(array[%s])" % ",".join("'%s'" % t for t in include_set_iter)
    query = (
        "select table_name from information_schema.tables where table_schema = 'public' and table_name like any %s"
        % like_query
    )
    with connection.cursor() as cursor:
        cursor.execute(query)
        rows = cursor.fetchall()
        rows = list(map(lambda x: x[0], rows))
    return rows


if __name__ == "__main__":
    rows = select_tables()
    open("diesel.toml", "w+").write(
        "[print_schema]\nfilter = { only_tables = [%s] }\n"
        % ", ".join('"%s"' % (t,) for t in rows)
    )
