app "effect-example"
    packages { base: "platform" }
    imports [ base.Task.{ Task, after } ]
    provides [ main ] to base

main : Task.Task {} I64
main =
    Task.succeed {}