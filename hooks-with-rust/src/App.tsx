import React, { Component, FormEvent, useState, ChangeEvent } from "react"
import { InputGroup, Button, FormGroup } from "@blueprintjs/core"
import styles from "./App.module.css"
import axios from "axios"

function App() {
  const [user, setUser] = useState({
    nickname: "",
    password: "",
    password_confirm: ""
  })

  const onSubmit = async (event: FormEvent) => {
    event.preventDefault()
    const data = await axios.post("/signup", user)
    console.log(data)
  }

  return (
    <div className="App">
      <form onSubmit={onSubmit} className={styles.signupForm}>
        <FormGroup inline>
          <InputGroup
            leftIcon="user"
            large
            value={user.nickname}
            onChange={(event: ChangeEvent<HTMLInputElement>) =>
              setUser({
                ...user,
                nickname: event.target.value
              })
            }
          />
        </FormGroup>
        <FormGroup inline>
          <InputGroup
            leftIcon="lock"
            large
            value={user.password}
            onChange={(event: ChangeEvent<HTMLInputElement>) =>
              setUser({
                ...user,
                password: event.target.value
              })
            }
          />
        </FormGroup>
        <FormGroup inline>
          <InputGroup
            leftIcon="lock"
            large
            value={user.password_confirm}
            onChange={(event: ChangeEvent<HTMLInputElement>) =>
              setUser({
                ...user,
                password_confirm: event.target.value
              })
            }
          />
        </FormGroup>
        <FormGroup inline>
          <Button large type="submit">
            提交
          </Button>
        </FormGroup>
      </form>
    </div>
  )
}
export default App
