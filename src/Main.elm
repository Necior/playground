module Main exposing (..)

import Browser
import Html exposing (Html, button, div, h1, h2, input, li, small, text, ul)
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
        ([ h1 [] [ text "Simple Todo App" ]
         , input [ onInput MessageChanged, value model.currentTodo ] []
         , button [ onClick AddTodo ] [ text "Add todo" ]
         ]
            ++ viewTodos model
        )


viewTodos : Model -> List (Html Msg)
viewTodos model =
    [ h2 [] [ text "Todo" ]
    , if List.length model.todos > 0 then
        ul []
            (List.map (\todo -> li [] [ text todo ]) model.todos)

      else
        small [] [ text "Hooray! Everything's done :-)" ]
    ]
