from django.conf.urls.defaults import *

from configure.views import *

urlpatterns = patterns('',
    (r'^$', setup),
    (r'^create_mgs/(?P<host_id>\w+)/$', create_mgs),
    (r'^create_filesystem/(?P<mgs_id>\w+)/$', create_fs),
    (r'^create_mds/(?P<host_id>\w+)/$', create_mds),
    (r'^create_oss/(?P<host_id>\w+)/$', create_oss),
    (r'^jobs/$', jobs),
    (r'^jobs_json/$', jobs_json),
    (r'^job/(?P<job_id>\w+)/$', job),
    (r'^states/$', states),
    (r'^set_state/(?P<content_type_id>\w+)/(?P<stateful_object_id>\w+)/(?P<new_state>\w+)/$', set_state),
    (r'^filesystem/(?P<filesystem_id>\w+)/$', filesystem),
)
