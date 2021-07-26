module Main exposing (..)

import Browser
import Html exposing (Html, button, div, input, p, text)
import Html.Attributes exposing (value)
import Html.Events exposing (onClick, onInput)



-- MAIN


main =
    Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type alias Model =
    { currentTodo : String
    , todos : List String
    }


init : Model
init =
    { currentTodo = ""
    , todos = []
    }



-- UPDATE


type Msg
    = MessageChanged String
    | AddTodo


update : Msg -> Model -> Model
update msg model =
    case msg of
        MessageChanged t ->
            { model | currentTodo = t }

        AddTodo ->
            { model | currentTodo = "", todos = model.todos ++ [ model.currentTodo ] }



-- VIEW


view : Model -> Html Msg
view model =
    div []
        [ input [ onInput MessageChanged, value model.currentTodo ] []
        , button [ onClick AddTodo ] [ text "Add todo" ]
        , div []
            (List.map (\todo -> p [] [ text todo ]) model.todos)
        ]
