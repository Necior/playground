module Main exposing (..)

import Browser
import Browser.Navigation as Nav
import Html exposing (Attribute, Html, button, del, h1, h2, h3, input, li, pre, small, text, ul)
import Html.Attributes exposing (autofocus, value)
import Html.Events exposing (keyCode, on, onClick, onInput, onMouseOut, onMouseOver)
import Json.Decode as D
import Json.Encode as E
import Url



-- MAIN


main =
    Browser.application
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        , onUrlChange = UrlChanged
        , onUrlRequest = LinkClicked
        }



-- MODEL


type alias Todo =
    { title : String
    }


type alias Model =
    { currentTodo : String
    , todos : List Todo
    , done : List Todo
    , hovered : Maybe Todo
    }


init : () -> Url.Url -> Nav.Key -> ( Model, Cmd Msg )
init flags url key =
    ( { currentTodo = ""
      , todos = []
      , done = []
      , hovered = Nothing
      }
    , Cmd.none
    )



-- UPDATE


type Msg
    = MessageChanged String
    | AddTodo
    | MarkAsDone Todo
    | UrlChanged Url.Url
    | LinkClicked Browser.UrlRequest
    | MouseOver Todo
    | MouseOut


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        MessageChanged t ->
            ( { model | currentTodo = t }, Cmd.none )

        AddTodo ->
            if model.currentTodo == "" then
                ( model, Cmd.none )

            else
                ( { model | currentTodo = "", todos = model.todos ++ [ { title = model.currentTodo } ] }
                , Cmd.none
                )

        MarkAsDone t ->
            ( { model
                | todos = List.filter (\task -> task /= t) model.todos
                , done = List.append (List.filter (\task -> task == t) model.todos) model.done
              }
            , Cmd.none
            )

        UrlChanged url ->
            ( model, Cmd.none )

        LinkClicked urlRequest ->
            ( model, Cmd.none )

        MouseOver t ->
            ( { model | hovered = Just t }, Cmd.none )

        MouseOut ->
            ( { model | hovered = Nothing }, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



-- VIEW


view : Model -> Browser.Document Msg
view model =
    let
        body =
            [ h1 [] [ text "Simple Todo App" ]
            , input [ autofocus True, onInput MessageChanged, value model.currentTodo, onEnter AddTodo ] []
            , button [ onClick AddTodo ] [ text "Add todo" ]
            ]
                ++ viewTodos model
                ++ [ h3 [] [ text "serialized" ]
                   , pre [] [ text (E.encode 2 (encode model)) ]
                   ]
    in
    { title = title (List.length model.todos)
    , body = body
    }


title : Int -> String
title taskCount =
    let
        appName =
            "Simple Todo App"
    in
    if taskCount == 0 then
        appName

    else
        "(" ++ String.fromInt taskCount ++ ") " ++ appName


viewTodos : Model -> List (Html Msg)
viewTodos model =
    [ h2 [] [ text "Todo" ]
    , if List.length model.todos > 0 then
        ul []
            (List.map
                (\t ->
                    viewTodo t (Just t == model.hovered)
                )
                model.todos
            )

      else
        small [] [ text "Hooray! Everything's done :-)" ]
    , h2 [] [ text "Done" ]
    , if List.length model.done > 0 then
        ul []
            (List.map (\todo -> li [] [ small [] [ del [] [ text todo.title ] ] ]) model.done)

      else
        small [] [ text "Get back to work! :-)" ]
    ]


viewTodo : Todo -> Bool -> Html Msg
viewTodo todo hovered =
    li [ onMouseOver (MouseOver todo), onMouseOut MouseOut, onClick (MarkAsDone todo) ]
        [ if hovered then
            del [] [ text todo.title ]

          else
            text todo.title
        ]


encode : Model -> E.Value
encode model =
    E.object
        [ ( "todos", E.list encodeTodo model.todos )
        , ( "done", E.list encodeTodo model.done )
        ]


encodeTodo : Todo -> E.Value
encodeTodo todo =
    E.object [ ( "title", E.string todo.title ) ]



-- stolen from BSD-licensed Evan Czaplicki's TodoMVC
-- https://github.com/evancz/elm-todomvc/blob/f236e7e56941c7705aba6e42cb020ff515fe3290/src/Main.elm#L248-L257


onEnter : Msg -> Attribute Msg
onEnter msg =
    let
        isEnter code =
            if code == 13 then
                D.succeed msg

            else
                D.fail "not Enter"
    in
    on "keydown" (D.andThen isEnter keyCode)
