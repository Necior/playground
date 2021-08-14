module Main exposing (..)

import Browser
import Browser.Navigation as Nav
import Html exposing (Attribute, Html, button, del, div, h1, h2, input, li, small, text, ul)
import Html.Attributes exposing (autofocus, value)
import Html.Events exposing (keyCode, on, onClick, onInput)
import Json.Decode
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


type alias Model =
    { currentTodo : String
    , todos : List String
    , done : List String
    }


init : () -> Url.Url -> Nav.Key -> ( Model, Cmd Msg )
init flags url key =
    ( { currentTodo = ""
      , todos = []
      , done = []
      }
    , Cmd.none
    )



-- UPDATE


type Msg
    = MessageChanged String
    | AddTodo
    | MarkAsDone String
    | UrlChanged Url.Url
    | LinkClicked Browser.UrlRequest


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        MessageChanged t ->
            ( { model | currentTodo = t }, Cmd.none )

        AddTodo ->
            if model.currentTodo == "" then
                ( model, Cmd.none )

            else
                ( { model | currentTodo = "", todos = model.currentTodo :: model.todos }
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



-- SUBSCRIPTIONS


subscriptions : Model -> Sub msg
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
            (List.map (\todo -> li [] [ button [ onClick (MarkAsDone todo) ] [ text "Done" ], text (" " ++ todo) ]) model.todos)

      else
        small [] [ text "Hooray! Everything's done :-)" ]
    , h2 [] [ text "Done" ]
    , if List.length model.done > 0 then
        ul []
            (List.map (\todo -> li [] [ small [] [ del [] [ text todo ] ] ]) model.done)

      else
        small [] [ text "(finished tasks will appear here)" ]
    ]



-- stolen from BSD-licensed Evan Czaplicki's TodoMVC
-- https://github.com/evancz/elm-todomvc/blob/f236e7e56941c7705aba6e42cb020ff515fe3290/src/Main.elm#L248-L257


onEnter : Msg -> Attribute Msg
onEnter msg =
    let
        isEnter code =
            if code == 13 then
                Json.Decode.succeed msg

            else
                Json.Decode.fail "not Enter"
    in
    on "keydown" (Json.Decode.andThen isEnter keyCode)
