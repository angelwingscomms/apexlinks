# For AI agent

## structure
  subject codes
    "Math": "m",
    "English Language": "e",
    "Basic Science and Technology": "bst",
    "Computer": "c",
    "History": "h",
    "Physical and Health Education": "phe",
    "National Values": "nv",
    "Cultural and Creative Arts": "cca",
    "PreVocational Studies": "pvs",
    "French": "f",
    "Religious Studies": "rs",
    "Music": "ms"

  db user object
    s: 'u' // constant - tenant id
    n: string // user's name
    e: string // user email
    g: string // google id

  db school object
    s: 'sch' // constant - tenant id
    n: string // name
    y: JSON map of numbers to strings, with an extra key called `next_key` which has a number value // academic years the school has

  db school_user object
    s: 'sch_usr' // tenant id
    r: string array // roles
    n: string // name
    u: uuid // user
    sc: uuid // school

  db score object
    s: 'sch_scr' // tenant id
    u: uuid // school_user uuid
    t: i8 // valid: 1-3
    y: i8 // academic year key - maps to a y key in school_object
    y_: i8 // academic session e.g 2014
    j: string // subject code 
    1: i8 // 1st CA (continous assessment)
    2: i8 // 2nd CA
    3: i8 // 3rd CA 
    p: i8 // project
    e: i8 // exam

  access request object
    u: uuid // user requesting access
    n: name // name of user requesting access
    s: 'sch_aro' // tenant id
    r: boolean // rejected


## todo
  login w/ google
    if user doesn't exist, stores user name and email and google id
  pages
    dashboard home (/d/)
      2 buttons: create school, request access to a school
    create school
      the user creates a school with a name, which creates a school object, then a school_user object with with `r`: ['admin'] (admin role); goes to edit school page
    edit school
      name edit
      add/remove academic years from list of year names - edits JSON object from `y` key on school object
        when year is removed, remove the key for that year from the object, removing the entire year's entry from the object
        when year is added, use value of `next_key` for the new year's key
    request access
      user requests access to the school portal as a student or teacher or admin role
      it shows the user's Google account name, telling them that the school wants to use their full name, asking them if to  continue with that name or if to use a different name, if yes, it shows an input box for a different name
      on the db, an access request object is created, if a different name was provided by the user, that name is used in the access request object
    view access requests
      only loads if user is admin
      lists all access requests that have not been rejected, shows name and role with grant or reject icon buttons for each listing
      grant
        create school user object in db, with `r`: ['student'] (student role)
        delete access request object from db
      reject
        set access request object r field to true
    add score /s/a
      only loads if user is admin or teacher
      search for student button
        combobox school search by name, enter key runs search, search runs on server, returns 3 closest results
        combobox school_user search by name (filter by tenant id `s` = `sch_usr`), enter key runs search, search runs on server, returns 3 closest results
      keys to input: t (select from 1, 2 or 3), y (select from), y_, j - required; 1, 2, 3, p, e - optional
    score detail
      shows school user name
      shows values of keys t, y, y_ from score object. shows subject mapped from `j` key. shows 1, 2, 3, p, e - if not empty