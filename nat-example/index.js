
import init from 'desk-client'
setTimeout(function(){
    init()
},2000)

import {init_pkg_ws} from 'pkg_ws'
init_pkg_ws()
export * from 'pkg_ws'